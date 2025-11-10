use crate::cpu::Cpu;
use anyhow::Result;
use crossterm::cursor::Show;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{event, event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};
use std::io::{stdout, Stdout};
use std::time::Duration;
use std::{collections::VecDeque, time::Instant};

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

        let mut stdout = stdout();

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

    pub fn tick(&mut self, cpu: &mut Cpu) -> Result<()> {
        // draw
        self.terminal.draw(|f| {
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(6),
                    Constraint::Length(4),
                    Constraint::Fill(1),
                ])
                .split(f.area());

            let inner_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Min(20),
                    Constraint::Min(44),
                ])
                .split(outer_layout[0]);

            // Logs
            let log_text = self.logs.iter()
                .rev().take(200).cloned().collect::<Vec<_>>().join("\n");

            let logs = Paragraph::new(log_text)
                .block(Block::default().borders(Borders::ALL).title(" Logs "));

            f.render_widget(logs, outer_layout[2]);

            let mut cpu_values: Vec<Row> = Vec::new();
            cpu_values.push(Row::new(vec![
                Cell::from(" PC"), Cell::from(format!("{:#05X}", cpu.pc)),
                Cell::from("OPCODE"), Cell::from(format!("{:#05X}", 0x289)),
            ]));
            cpu_values.push(Row::new(vec![
                Cell::from(" SP"), Cell::from(format!("{}", cpu.sp)),
                Cell::from("DRAW"), Cell::from(format!("{}", if cpu.draw_flag { "□" } else { "■" })),
            ]));
            cpu_values.push(Row::new(vec![
                Cell::from(" I"), Cell::from(format!("{:#05X}", cpu.i)),
                Cell::from("PANIC!"), Cell::from(format!("{}", if cpu.panic { "□" } else { "■" })),
            ]));

            let cpu_var = Table::new(cpu_values, [
                Constraint::Length(3), Constraint::Length(6),
                Constraint::Length(6), Constraint::Length(6),
            ]).block(Block::default().borders(Borders::ALL).title(" CPU "));

            f.render_widget(cpu_var, inner_layout[0]);

            let mut regs_values: Vec<Row> = Vec::new();
            for r in 0..4 {
                let mut cells = Vec::new();
                for c in 0..4 {
                    let idx = r + c * 4;
                    cells.push(Cell::from(format!(" V{:#02}:", idx)));
                    cells.push(Cell::from(format!("{:#02}", cpu.v[idx])));
                }
                regs_values.push(Row::new(cells));
            }

            let regs = Table::new(regs_values, [
                Constraint::Length(4), Constraint::Length(2),
                Constraint::Length(4), Constraint::Length(2),
                Constraint::Length(4), Constraint::Length(2),
                Constraint::Length(4), Constraint::Length(2),
            ]).block(Block::default().borders(Borders::ALL).title(" Regs "));

            f.render_widget(regs, inner_layout[1]);

            // FPS
            let mut avg = 0.0;

            if !self.fps_history.is_empty() {
                avg = self.fps_history.iter().copied().sum::<f32>() / (self.fps_history.len() as f32);
            }

            self.fps_history.push_front((1_000_000 / self.last_frame.elapsed().as_micros()) as f32);
            self.last_frame = Instant::now();
            let fps_text = format!(
                "FPS avg: {:>5.1}\nLast {} frame:\n{}",
                avg,
                self.fps_history.len(),
                ascii_sparkline(&self.fps_history, 10.0, 120.0)
            );
            let fps = Paragraph::new(fps_text)
                .block(Block::default().borders(Borders::ALL).title(" Performance "));
            f.render_widget(fps, outer_layout[1]);
        })?;

        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(k) = event::read()? {
                if k.code == KeyCode::Char('q') || k.code == KeyCode::Esc {
                    cpu.running = false;
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

    pub fn quit(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen, Show, DisableMouseCapture, Clear(ClearType::All))?;
        Ok(())
    }
}

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
