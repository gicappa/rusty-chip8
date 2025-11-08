use crate::cpu::Cpu;
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use crossterm::{event, event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};
use std::io::Stdout;
use std::time::Duration;
use std::{collections::VecDeque, io, time::Instant};

pub struct CpuDebugger {
    logs: Vec<String>,
    fps_history: VecDeque<f32>,
    last_frame: Instant,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl CpuDebugger {
    pub fn new() -> Self {

        // setup terminal
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        Self {
            logs: Vec::with_capacity(2000),
            fps_history: VecDeque::with_capacity(120),
            last_frame: Instant::now(),
            terminal,
        }
    }

    fn push_log(&mut self, line: String) {
        self.logs.push(line);
        if self.logs.len() > 500 {
            let excess = self.logs.len() - 500;
            self.logs.drain(0..excess);
        }
    }

    pub fn tick(&mut self, cpu: &Cpu) -> Result<()> {
        // draw
        self.terminal.draw(|f| {
            let size = f.area();

            // horizontal layout: 50% log | 30% register | 20% fps
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(30),
                    Constraint::Percentage(20),
                ])
                .split(size);

            // Logs
            let log_text = self.logs.iter().rev().take(200).cloned().collect::<Vec<_>>().join("\n");
            let logs = Paragraph::new(log_text)
                .block(Block::default().borders(Borders::ALL).title(" Logs "));
            f.render_widget(logs, chunks[0]);

            // Registers
            let mut rows: Vec<Row> = Vec::new();
            rows.push(Row::new(vec![
                Cell::from("PC"), Cell::from(format!("{:#06X}", cpu.pc)),
                Cell::from("SP"), Cell::from(format!("{}", cpu.sp)),
                Cell::from(" I"), Cell::from(format!("{:#06X}", cpu.i)),
            ]));

            for r in 0..2 {
                let mut cells = Vec::new();
                for c in 0..8 {
                    let idx = r * 8 + c;
                    cells.push(Cell::from(format!("V{:X}", idx)));
                    cells.push(Cell::from(format!("{:#04X}", cpu.v[idx])));
                }
                rows.push(Row::new(cells));
            }

            let regs = Table::new(rows, [
                Constraint::Length(3), Constraint::Length(8),
                Constraint::Length(3), Constraint::Length(8),
                Constraint::Length(2), Constraint::Length(8),
            ])
                .block(Block::default().borders(Borders::ALL).title(" CPU "));

            f.render_widget(regs, chunks[1]);

            // FPS
            // let avg = self.avg_fps(); error!!!
            let fps_text = format!(
                "FPS avg: {:>5.1}\nLast {} frame:\n{}",
                10,
                self.fps_history.len(),
                ascii_sparkline(&self.fps_history, 10.0, 120.0)
            );
            let fps = Paragraph::new(fps_text)
                .block(Block::default().borders(Borders::ALL).title(" Performance "));
            f.render_widget(fps, chunks[2]);
        })?;
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(k) = event::read()? {
                if k.code == KeyCode::Char('q') || k.code == KeyCode::Esc {
                    break;
                }
            }
        }

        Ok(())
    }

    fn avg_fps(&self) -> f32 {
        if self.fps_history.is_empty() { return 0.0; }
        self.fps_history.iter().copied().sum::<f32>() / (self.fps_history.len() as f32)
    }

    fn quit(&mut self) -> Result<()> {
        // let res = run_app(&mut terminal, rx);
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;

        // res
        Ok(())
    }
}

// Sparklines ASCII super semplici; min/max clampati
fn ascii_sparkline(hist: &VecDeque<f32>, min: f32, max: f32) -> String {
    const BARS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let mut s = String::new();
    for &v in hist {
        let x = v.clamp(min, max);
        let t = ((x - min) / (max - min + f32::EPSILON)) * (BARS.len() as f32 - 1.0);
        s.push(BARS[t.round() as usize]);
    }
    s
}
