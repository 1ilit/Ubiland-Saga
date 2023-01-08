use glium::{implement_vertex, uniform, Display, Surface};

pub const SCREEN_HEIGHT: f32 = 576.0; // 12
pub const SCREEN_WIDTH: f32 = 768.0; // 16
pub const BOTTOM: f32 = -SCREEN_HEIGHT / 2.0;
pub const TOP: f32 = SCREEN_HEIGHT / 2.0;
pub const LEFT: f32 = -SCREEN_WIDTH / 2.0;
pub const RIGHT: f32 = SCREEN_WIDTH / 2.0;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, color, tex_coords);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,

    pub vertex_array: Vec<Vertex>,
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub index_buffer: glium::index::NoIndices,
    pub matrix: [[f32; 4]; 4],
}

impl Rectangle {
    pub fn new(display: &glium::Display, w: u32, h: u32) -> Self {
        let x = ((w as f32) * 2. / SCREEN_WIDTH) / 2.;
        let y = ((h as f32) * 2. / SCREEN_HEIGHT) / 2.;

        let vertex1 = Vertex {
            //btm right
            position: [x, -y],
            color: [0.0, 0.0, 1.0, 1.0],
            tex_coords: [1.0, 0.0],
        };
        let vertex2 = Vertex {
            // top right
            position: [x, y],
            color: [0.0, 1.0, 0.0, 1.0],
            tex_coords: [1.0, 1.0],
        };
        let vertex3 = Vertex {
            //btm left
            position: [-x, -y],
            color: [1.0, 0.0, 1.0, 1.0],
            tex_coords: [0.0, 0.0],
        };
        let vertex4 = Vertex {
            //top left
            position: [-x, y],
            color: [0.0, 0.0, 1.0, 1.0],
            tex_coords: [0.0, 1.0],
        };

        let shape = vec![vertex1, vertex2, vertex3, vertex4];
        Self {
            width: w as f32,
            height: h as f32,
            vertex_array: shape.clone(),
            vertex_buffer: glium::VertexBuffer::new(display, &shape).unwrap(),
            index_buffer: glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
        }
    }

    pub fn set_color(&mut self, display: &Display, color: [f32; 4]) {
        for i in 0..4 {
            self.vertex_array[i].color = color;
        }
        self.vertex_buffer = glium::VertexBuffer::new(display, &self.vertex_array).unwrap();
    }

    pub fn set_gradient(&mut self, display: &Display, c1: [f32; 4], c2: [f32; 4], dir: Direction) {
        match dir {
            Direction::Horizontal => {
                self.vertex_array[0].color = c1;
                self.vertex_array[1].color = c1;
                self.vertex_array[2].color = c2;
                self.vertex_array[3].color = c2;
            }
            Direction::Vertical => {
                self.vertex_array[0].color = c1;
                self.vertex_array[2].color = c1;
                self.vertex_array[1].color = c2;
                self.vertex_array[3].color = c2;
            }
        }
        self.vertex_buffer = glium::VertexBuffer::new(display, &self.vertex_array).unwrap();
    }

    pub fn flip_tex_coords(&mut self, display: &Display, dir: Direction){
        match dir {
            Direction::Horizontal => {
                // 0-2, 1-3
                let temp=self.vertex_array[0].tex_coords;
                self.vertex_array[0].tex_coords=self.vertex_array[2].tex_coords;
                self.vertex_array[2].tex_coords=temp;

                let temp=self.vertex_array[1].tex_coords;
                self.vertex_array[1].tex_coords=self.vertex_array[3].tex_coords;
                self.vertex_array[3].tex_coords=temp;
            }
            Direction::Vertical => {
                // 0-1 2-3
                let temp=self.vertex_array[0].tex_coords;
                self.vertex_array[0].tex_coords=self.vertex_array[1].tex_coords;
                self.vertex_array[1].tex_coords=temp;

                let temp=self.vertex_array[2].tex_coords;
                self.vertex_array[2].tex_coords=self.vertex_array[3].tex_coords;
                self.vertex_array[3].tex_coords=temp;
            }
        }
        self.vertex_buffer = glium::VertexBuffer::new(display, &self.vertex_array).unwrap();
    }

    pub fn scale(&mut self, factor: f32) {
        self.matrix[0][0] *= factor;
        self.matrix[1][1] *= factor;
        self.matrix[2][2] *= factor;

        self.height *= factor;
        self.width *= factor;
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        let x0 = x * 2. / SCREEN_WIDTH;
        let y0 = y * 2. / SCREEN_HEIGHT;

        self.matrix[3][0] += x0;
        self.matrix[3][1] += y0;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.set_x(x);
        self.set_y(y);
    }

    pub fn set_x(&mut self, x: f32){
        let x0 = x * 2. / SCREEN_WIDTH;
        self.matrix[3][0] = x0;
    }

    pub fn set_y(&mut self, y: f32){
        let y0 = y * 2. / SCREEN_HEIGHT;
        self.matrix[3][1] = y0;
    }

    pub fn get_position(&mut self) -> (f32, f32) {
        (self.matrix[3][0] * RIGHT, self.matrix[3][1] * TOP)
    }

    pub fn draw(&self, target: &mut glium::Frame, program: &glium::Program) {
        let uniforms = uniform! {
            isTex: false,
            matrix: self.matrix,
        };
        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
