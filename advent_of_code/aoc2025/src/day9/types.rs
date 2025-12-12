use std::{cmp::{max, min}, fmt::Display};

#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Green,
}

#[derive(Debug)]
pub struct Grid {
    rows: u32,
    columns: u32,
    data: Vec<Option<Color>>,
}

impl Grid {
    pub fn new(rows: u32, columns: u32) -> Self {
        Grid {
            rows,
            columns,
            data: vec![None; rows  as usize * columns as usize],
        }
    }
    
    fn paint(&mut self, x: u32, y: u32, new_color: Color) {
        let idx = (y * self.columns + x) as usize;
        if let Some(existing_color) = self.data[idx] {
            if existing_color != new_color {
                panic!("Cannot paint over existing color");
            }
        }
        self.data[idx] = Some(new_color);
    }
    
    fn paint_line(&mut self, p1: &Point, p2: &Point, color: Color) {
        if p1.x == p2.x {
            let x = p1.x;
            let y_start = min(p1.y, p2.y) + 1;
            let y_end = max(p1.y, p2.y);
            for y in y_start..y_end {
                self.paint(x, y, color);
            }
            return;
        }
        if p1.y == p2.y {
            let y = p1.y;
            let x_start = min(p1.x, p2.x) + 1;
            let x_end = max(p1.x, p2.x);
            for x in x_start..x_end {
                self.paint(x, y, color);
            }
            return;
        }
        panic!("Cannot paint diagonal lines. Points {:?} and {:?} are not aligned", p1, p2);
    }
    
    pub fn connect_points_green(&mut self, points: &[Point]) {
        for i in 0..points.len() {
            let p1 = &points[i];
            let p2 = &points[(i + 1) % points.len()];
            self.paint(p1.x, p1.y, Color::Red);
            self.paint(p2.x, p2.y, Color::Red);
            self.paint_line(p1, p2, Color::Green);
        }
    }
    
    pub fn paint_inside_green(&mut self, x: u32, y: u32) {
        let idx = (y * self.columns + x) as usize;
        
        match self.data.get(idx) {
            None => {
                // Outta bounds: do nothing
                return;
            }
            Some(point_color) => {
                match point_color {
                    None => {
                        // Not painted yet: paint it
                        self.paint(x, y, Color::Green);
                        // Attempt to paint surrounding points
                        self.paint_inside_green(x+1, y);
                        self.paint_inside_green(x, y+1);
                        if x > 0 {
                            self.paint_inside_green(x-1, y);
                        }
                        if y > 0 {
                            self.paint_inside_green(x, y-1);
                        }
                    },
                    _ => {
                        // Already painted: do nothing
                        return;
                    }
                }
            }
        }  
    }
    
    pub fn rectangle_is_inside(&self, p1: &Point, p2: &Point) -> bool {
        let x_start = min(p1.x, p2.x) + 1;
        let x_end = max(p1.x, p2.x);
        let y_start = min(p1.y, p2.y) + 1;
        let y_end = max(p1.y, p2.y);
        
        for x in x_start..x_end {
            for y in y_start..y_end {
                if self.data[(y * self.columns + x) as usize].is_none() {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.columns {
                let idx = (y * self.columns + x) as usize;
                let tile = match self.data[idx] {
                    None => '.',
                    Some(color) => match color {
                        Color::Red => '#',
                        Color::Green => 'X',
                    }
                };
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
