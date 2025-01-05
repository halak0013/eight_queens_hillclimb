use egui::Pos2;
use rand::{rngs::StdRng, seq::SliceRandom, thread_rng, SeedableRng};

use crate::algo::{Checker, HillClimbing};

#[derive(Debug)]
pub struct QApp {
    board: Vec<Vec<bool>>,
    checker: Checker,
    hill_climbing: HillClimbing,
    i: usize,
    j: usize,
    tmp: Tm,
}

#[derive(Debug)]
struct Tm {
    a: usize,
    d: usize,
    hw: usize,
    scores: Vec<usize>,
    previous_scores: Vec<usize>,
    is_solved: bool,
}
const OFFSET: usize = 40;
const SIZE: usize = 50;
const SIZE_2: f32 = 25.5;
impl Default for QApp {
    fn default() -> Self {
        let mut data: Vec<usize> = (0..8).collect();
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);
        data.shuffle(&mut rng);

        let mut matrix: Vec<Vec<bool>> = vec![vec![false; 8]; 8];
        for i in 0..matrix.len() {
            matrix[i][data[i]] = true;
        }

        println!("{:?}", matrix);

        Self {
            board: matrix,
            checker: Checker,
            hill_climbing: HillClimbing,
            i: 0,
            j: 0,
            tmp: Tm {
                a: 0,
                d: 0,
                hw: 0,
                scores: vec![],
                previous_scores: vec![],
                is_solved: false,
            },
        }
    }
}

impl eframe::App for QApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::right("info").show(ctx, |ui| {
            ui.heading("Info");
            ui.add(egui::Slider::new(&mut self.i, 0..=8).text("Satır"));
            ui.add(egui::Slider::new(&mut self.j, 0..=8).text("Sütun"));
            if ui.button("Hesapla").clicked() {
                self.tmp.a = self.checker.check_all(&self.board, self.i, self.j);
                self.tmp.d = self.checker.check_diagonal(&self.board, self.i, self.j);
                self.tmp.hw = self.checker.checek_hw2(&self.board, self.i, self.j);
                println!("{:?}", self.tmp);
            }
            ui.label(format!(
                "a: {} d:{} hw:{}",
                self.tmp.a, self.tmp.d, self.tmp.hw
            ));
            if ui.button("Karıştır").clicked() {
                self.shuffle();
            }
            ui.separator();
            ui.label("Hill Climbing Algorithm");
            if ui.button("Calculate").clicked() {
                self.tmp.scores = self.hill_climbing.find_best_all(&mut self.board);

                self.tmp.is_solved = self.tmp.scores.iter().sum::<usize>() == 0;

                if self.tmp.scores == self.tmp.previous_scores {
                    self.shuffle();
                    println!("Shuffled");
                }
                self.tmp.previous_scores = self.tmp.scores.clone();
            }
            ui.label(format!("{:?}", self.tmp.scores));
            ui.label(format!(
                "{} Solved",
                if self.tmp.is_solved { "" } else { "Not" }
            ));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            for (i, v) in self.board.iter().enumerate() {
                for (j, d) in v.iter().enumerate() {
                    self.draw_tiles(ui, i, j);
                    if *d {
                        self.draw_queens(ui, i, j);
                    }
                }
            }
        });
    }
}

impl QApp {
    fn shuffle(&mut self) {
        let mut data: Vec<usize> = (0..8).collect();
        let mut rng = thread_rng();
        data.shuffle(&mut rng);

        let mut matrix: Vec<Vec<bool>> = vec![vec![false; 8]; 8];
        for i in 0..matrix.len() {
            matrix[i][data[i]] = true;
        }
        self.board = matrix;
    }
    fn draw_tiles(&self, ui: &mut egui::Ui, i: usize, j: usize) {
        let node_rect = egui::Rect::from_center_size(
            self.get_pos(i, j),
            egui::Vec2::new(SIZE as f32, SIZE as f32),
        );

        ui.painter()
            .rect_filled(node_rect, 1., self.get_color(i, j));
        ui.painter().text(
            self.get_pos(i, j),
            egui::Align2::CENTER_CENTER,
            format!("{} {}", i, j),
            egui::FontId::proportional(13.0),
            self.get_color(i, j + 1),
        );
    }
    fn draw_queens(&self, ui: &mut egui::Ui, i: usize, j: usize) {
        let node_rect =
            egui::Rect::from_center_size(self.get_pos(i, j), egui::Vec2::new(SIZE_2, SIZE_2));

        ui.painter().rect_filled(node_rect, 80., egui::Color32::RED);
    }

    fn get_pos(&self, i: usize, j: usize) -> Pos2 {
        Pos2::new((i * SIZE + OFFSET) as f32, (j * SIZE + OFFSET) as f32)
    }
    fn get_color(&self, a: usize, b: usize) -> egui::Color32 {
        if (a + b) % 2 == 0 {
            egui::Color32::BLACK
        } else {
            egui::Color32::WHITE
        }
    }
}
