//the coding style will be a bit inconsistent as I'm learning parts of the syntax as I go. maybe one day I will go though here and fix anything rustfmt does not grab
use crate::font::FONT_SET;
extern crate sdl2;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
/*
struct IO<'a> {
    sdl_context: Sdl,
    event_pump: EventPump,
    display: Display<'a>,
    keypad: Keypad<'a>,
}

impl IO<'_> {
    pub fn new() -> Self {

        let sdl_context = sdl2::init().unwrap();
        let event_p = sdl_context.event_pump().unwrap();
        IO {
            sdl_context: sdl_context,
            event_pump: event_p, 
            display: Display::new(sdl_context ,&event_p),
            keypad: Keypad::new(sdl_context, &event_p),
        }
    }

    pub fn update(&self) {
        self.display.update();
        self.keypad.update();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    panic!("game ended by escape code");
                },
                _ => {}
            }
        }
    }
}

struct Keypad<'keypad> {
    waiting: bool,
    keypad: bool,
    keys: [bool; 16],
    event_pump: &'keypad EventPump,
}

impl<'keypad> Keypad<'keypad> {
    pub fn new(sdl_context: Sdl, event_pump: &'keypad EventPump) -> Self {
        //might get rid of waiting
        Keypad {
            waiting: false,
            keypad: true,
            keys: [false; 16],
            event_pump: event_pump
        }
    }

    pub fn update(&mut self) {
        let board = self.event_pump.keyboard_state();

        //I did not see a way to iterate over this so time to disregard whoever said dont repeat yourself
        self.keys[0] = board.is_scancode_pressed(Scancode::Kp1);
        self.keys[1] = board.is_scancode_pressed(Scancode::Kp2);
        self.keys[2] = board.is_scancode_pressed(Scancode::Kp3);
        self.keys[3] = board.is_scancode_pressed(Scancode::Kp4);
        
        self.keys[4] = board.is_scancode_pressed(Scancode::Q);
        self.keys[5] = board.is_scancode_pressed(Scancode::W);
        self.keys[6] = board.is_scancode_pressed(Scancode::E);
        self.keys[7] = board.is_scancode_pressed(Scancode::R);
        
        self.keys[8] = board.is_scancode_pressed(Scancode::A);
        self.keys[9] = board.is_scancode_pressed(Scancode::S);
        self.keys[10] = board.is_scancode_pressed(Scancode::D);
        self.keys[11] = board.is_scancode_pressed(Scancode::F);

        self.keys[12] = board.is_scancode_pressed(Scancode::Z);
        self.keys[13] = board.is_scancode_pressed(Scancode::X);
        self.keys[14] = board.is_scancode_pressed(Scancode::C);
        self.keys[15] = board.is_scancode_pressed(Scancode::V);
    }
}


struct Display<'display> {
    vram: [[u8; 64]; 32],
    sdl_context: Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &'display EventPump,
}

impl<'display> Display<'display> {
    pub fn new(sdl_context: Sdl, event_pump: &'display sdl2::EventPump) -> Self {
        
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("chip-8 emulator", 320, 160)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window.into_canvas().build()
            .expect("could not make a canvas");
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        
        Display {
            vram: [[0; 64]; 32],
            canvas: canvas,
            event_pump: event_pump,
            sdl_context: sdl_context,
        }
    }





    pub fn clear(&mut self){
        self.vram = [[0; 64]; 32];
    }
    pub fn draw_pixel(&mut self, x: u8, y: u8){
        if self.vram[y as usize][x as usize] == 1{
            self.vram[y as usize][x as usize] = 0;
        }
        else{
            self.vram[y as usize][x as usize] = 1
        }
    }

    fn update(&mut self) {
        for i in 0..self.vram.len() {
            for j in 0..self.vram[i].len() {
                if self.vram[i][j] == 1 {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 0));
                }
                else {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                self.canvas.fill_rect(Rect::new((j as i32 * 5) as i32, (i as i32 * 5) as i32, (/*(x * 5) + */5) as u32, (/*(x * 5) + */5) as u32));
            }
        }
        self.canvas.present();
    }
}*/


struct IO {
    sdl_context: Sdl,
    event_pump: EventPump,
    vram: [[u8; 64]; 32],
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    keys: [bool; 16],
}
impl IO {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("chip-8 emulator", 320, 160)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window.into_canvas().build()
            .expect("could not make a canvas");
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        

        IO {
            sdl_context: sdl_context,
            event_pump: event_pump,
            canvas: canvas,
            vram: [[0; 64]; 32],
            keys: [false; 16],
        }
    }

    pub fn update(&mut self) {
        self.update_display();
        self.update_keypad();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    panic!("game ended by escape code");
                },
                _ => {}
            }
        }
    }


    
    pub fn clear_display(&mut self){
        self.vram = [[0; 64]; 32];
    }
    pub fn draw_pixel(&mut self, x: u8, y: u8){
        if self.vram[y as usize][x as usize] == 1{
            self.vram[y as usize][x as usize] = 0;
        }
        else{
            self.vram[y as usize][x as usize] = 1
        }
    }

    fn update_display(&mut self) {
        for i in 0..self.vram.len() {
            for j in 0..self.vram[i].len() {
                if self.vram[i][j] == 1 {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 0));
                }
                else {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                self.canvas.fill_rect(Rect::new((j as i32 * 5) as i32, (i as i32 * 5) as i32, (/*(x * 5) + */5) as u32, (/*(x * 5) + */5) as u32));
            }
        }
        self.canvas.present();
    }
    
    fn update_keypad(&mut self) {
        let board = self.event_pump.keyboard_state();

        //I did not see a way to iterate over this so time to disregard whoever said dont repeat yourself
        self.keys[0] = board.is_scancode_pressed(Scancode::Kp1);
        self.keys[1] = board.is_scancode_pressed(Scancode::Kp2);
        self.keys[2] = board.is_scancode_pressed(Scancode::Kp3);
        self.keys[3] = board.is_scancode_pressed(Scancode::Kp4);
        
        self.keys[4] = board.is_scancode_pressed(Scancode::Q);
        self.keys[5] = board.is_scancode_pressed(Scancode::W);
        self.keys[6] = board.is_scancode_pressed(Scancode::E);
        self.keys[7] = board.is_scancode_pressed(Scancode::R);
        
        self.keys[8] = board.is_scancode_pressed(Scancode::A);
        self.keys[9] = board.is_scancode_pressed(Scancode::S);
        self.keys[10] = board.is_scancode_pressed(Scancode::D);
        self.keys[11] = board.is_scancode_pressed(Scancode::F);

        self.keys[12] = board.is_scancode_pressed(Scancode::Z);
        self.keys[13] = board.is_scancode_pressed(Scancode::X);
        self.keys[14] = board.is_scancode_pressed(Scancode::C);
        self.keys[15] = board.is_scancode_pressed(Scancode::V);
    }


}

//program counter state
enum PcState {
    Next,
    Skip,
    Jump(usize),
} 

pub struct Processor {
    memory: [u8; 4096],
    registers: [u8; 16],
    index: u16,
    stack: [u16; 16],
    stack_pointer: u8,
    io: IO,
    delay_timer: u8,
    sound_timer: u8,
    opcode: u16,
    program_counter: u16,
}

impl Processor {
    pub fn new() -> Self {
        let mut ram = [0; 4096];

        for i in 0..FONT_SET.len(){
            ram[i] = FONT_SET[i];
        }

        Processor {
            memory: ram,
            registers: [0; 16],
            index: 0,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            io: IO::new(),
            program_counter: 0x200,
            opcode: 0,
        }
    }
    
    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            //println!("{:X}",byte);
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }

    pub fn tick(&mut self) {        

        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1
        }

        let opcode = ((self.memory[(self.program_counter) as usize] as u16) << 8 | (self.memory[(self.program_counter + 1) as usize]) as u16) as u16; 
        self.run_opcode(opcode);
    
        self.io.update();
    }

    fn run_opcode(&mut self, opcode: u16) {

        //println!("{:X}", opcode);
        let instruction = match opcode & 0xF000 {
            0x0000 => match opcode & 0x000f {
                0x0000 => self.op_00e0(&opcode),
                0x000e => self.op_00ee(&opcode),
                _ => panic!("unknown instruction under 0x0000 {:X}", opcode)
            }
            0x000e => self.op_00ee(&opcode),
            0x1000 => self.op_1nnn(&opcode),
            0x2000 => self.op_2nnn(&opcode),
            0x3000 => self.op_3xnn(&opcode),
            0x4000 => self.op_4xnn(&opcode),
            0x5000 => self.op_5xy0(&opcode),
            0x6000 => self.op_6xnn(&opcode),
            0x7000 => self.op_7xnn(&opcode),
            0x8000 => match opcode & 0x000f {
                0x0000 => self.op_8xy0(&opcode),
                0x0001 => self.op_8xy1(&opcode),
                0x0002 => self.op_8xy2(&opcode),
                0x0003 => self.op_8xy3(&opcode),
                0x0004 => self.op_8xy4(&opcode),
                0x0005 => self.op_8xy5(&opcode),
                0x0006 => self.op_8xy6(&opcode),
                0x0007 => self.op_8xy7(&opcode),
                0x000e => self.op_8xye(&opcode),
                _ => panic!("unknown instruction under 0x8000 {:X}", opcode)
            }
            0x9000 => self.op_9xy0(&opcode),
            0xa000 => self.op_annn(&opcode),
            0xb000 => self.op_bnnn(&opcode),
            0xc000 => self.op_cxnn(&opcode),
            0xd000 => self.op_dxyn(&opcode),
            0xe000 => match opcode & 0x00FF {
                0x009e => self.op_ex9e(&opcode),
                0x00ff => self.op_exa1(&opcode),
                _ => panic!("unknown instruction under 0xe000 {:X}", opcode)
            }
            0xf000 => match opcode & 0x00ff {
                0x0007 => self.op_fx07(&opcode),
                0x000a => self.op_fx0a(&opcode),
                0x0015 => self.op_fx15(&opcode),
                0x0018 => self.op_fx18(&opcode),
                0x001e => self.op_fx1e(&opcode),
                0x0029 => self.op_fx29(&opcode),
                0x0033 => self.op_fx33(&opcode),
                0x0055 => self.op_fx55(&opcode),
                0x0065 => self.op_fx65(&opcode),
                _ => panic!("unknown instruction under 0xf000 {:X}" , opcode)
            }

            _ => panic!("unknown instruction {:X}", opcode)
        };

        //have to increment the pc by two due to the fact that the instructions are 16 bit not 8 bit
        match instruction {
            PcState::Next => self.program_counter += 2,
            PcState::Skip => self.program_counter += 4,
            PcState::Jump(to) => self.program_counter = (to) as u16
        }
        
    }
    //0000 read by the reader but used to accses other instructions
    // 00E0 - Clear screen. 0000. 0000
    fn op_00e0(&mut self, opcode: &u16) -> PcState {
        self.io.clear_display();
        PcState::Next
    }
    // 00EE - Return from subroutine. 0000. 000e
    fn op_00ee(&mut self, opcode: &u16) -> PcState {
        self.stack_pointer -= 1;
        PcState::Jump((self.stack[self.stack_pointer as usize] + 1) as usize)
    }
    // 1NNN - Jumps to address NNN. 1000
    fn op_1nnn(&mut self, opcode: &u16) -> PcState {
        let destination = opcode & 0x0FFF;
        PcState::Jump(destination as usize)
    }
    //2NNN Calls subroutine at adderss NNN. 2000
    fn op_2nnn(&mut self, opcode: &u16) -> PcState {
        self.stack_pointer+=1;
        self.stack[(self.stack_pointer) as usize] = self.program_counter;
        let destination = opcode & 0x0FFF;
        PcState::Jump(destination as usize)
    }
    // 3XNN - Skips the next instruction if VX equals NN. 3000
    fn op_3xnn(&mut self, opcode: &u16) -> PcState {
        if (self.registers[((opcode & 0x0F00) >> 8) as usize] as u16 == (opcode) & (0x00FF)) {
            return PcState::Skip;
        }
        else {
            return PcState::Next;
        }
    }
    // 4XNN - Skips the next instruction if VX does not equal NN. 4000
    fn op_4xnn(&mut self, opcode: &u16) -> PcState {
        if (self.registers[((opcode & 0x0F00) >> 8) as usize] as u16 != (opcode) & (0x00FF)) {
            return PcState::Skip;
        }
        else {
            return PcState::Next;
        }
    }
    // 5XY0 - Skips the next instruction if VX equals VY. 5000
    fn op_5xy0(&mut self, opcode: &u16) -> PcState {
        if(self.registers[(opcode & 0x0F00 >> 8) as usize] == self.registers[((opcode & 0x00F0) >> 4) as usize]) {
            return PcState::Skip;
        }
        else {
            return PcState::Next;
        }
    }
    // 6XNN - Sets VX to NN. 6000
    fn op_6xnn(&mut self, opcode: &u16) -> PcState {
        self.registers[((opcode & 0x0F00) >> 8) as usize] = ((opcode) & (0x00FF)) as u8;
        PcState::Next
    }
    // 7XNN - Adds NN to VX. 7000
    fn op_7xnn(&mut self, opcode: &u16) -> PcState {
        let vx = self.registers[((opcode & 0x0F00) >> 8) as usize] as u16;
        let nn = ((opcode) & (0x00FF)) as u16;
        let solution = vx + nn;
        self.registers[((opcode & 0x0F00) >> 8) as usize] = solution as u8;
        PcState::Next
    }
    // 8XY_. 8000. functionless even though it is in the opcode reader

    // 8XY0 - Sets VX to the value of VY. 8000. 0000
    fn op_8xy0(&mut self, opcode: &u16) -> PcState {
        self.registers[(opcode & 0x0F00 >> 8) as usize] = self.registers[((opcode & 0x00F0) >> 4) as usize];
        PcState::Next
    }
    // 8XY1 - Sets VX to (VX OR VY). 8000. 0001
    fn op_8xy1(&mut self, opcode: &u16) -> PcState {
        self.registers[(opcode & 0x0F00 >> 8) as usize] = (self.registers[(opcode & 0x0F00 >> 8) as usize] | self.registers[((opcode & 0x00F0) >> 4) as usize]);
        PcState::Next
    }
    // 8XY2 - Sets VX to (VX AND VY). 8000. 0002
    fn op_8xy2(&mut self, opcode: &u16) -> PcState {
        self.registers[(opcode & 0x0F00 >> 8) as usize] = (self.registers[(opcode & 0x0F00 >> 8) as usize] & self.registers[((opcode & 0x00F0) >> 4) as usize]);
        PcState::Next
    }
    // 8XY3 - Sets VX to (VX XOR VY). 8000. 0003
    fn op_8xy3(&mut self, opcode: &u16) -> PcState {
        self.registers[(opcode & 0x0F00 >> 8) as usize] = (self.registers[(opcode & 0x0F00 >> 8) as usize] ^ self.registers[((opcode & 0x00F0) >> 4) as usize]);
        PcState::Next
    }
    // 8XY4 - Adds VY to VX. VF is set to 1 when there's a carry,
    // and to 0 when there isn't. 8000. 0004
    fn op_8xy4(&mut self, opcode: &u16) -> PcState {
        self.registers[(opcode & 0x0F00 >> 8) as usize] += self.registers[((opcode & 0x00F0) >> 4) as usize];
        if(self.registers[((opcode & 0x00F0) >> 4) as usize] > (0xFF - self.registers[((opcode & 0x0F00) >> 8) as usize])) {
            self.registers[0xF] = 1; //carry
        }
        else {
            self.registers[0xF] = 0;
        }
        PcState::Next
    }
    // 8XY5 - VY is subtracted from VX. VF is set to 0 when
    // there's a borrow, and 1 when there isn't. 8000. 0005
    fn op_8xy5(&mut self, opcode: &u16) -> PcState {
        if(self.registers[((opcode & 0x00F0) >> 4) as usize] > self.registers[((opcode & 0x0F00) >> 8) as usize]) {
            self.registers[0xF] = 0; // there is a borrow
        }
        else {
            self.registers[0xF] = 1;
        }
        self.registers[((opcode & 0x0F00) >> 8) as usize] -= self.registers[((opcode & 0x00F0) >> 4) as usize];
        PcState::Next
    }
    // 0x8XY6 - Shifts VX right by one. VF is set to the value of
    // the least significant bit of VX before the shift. 8000. 0006
    fn op_8xy6(&mut self, opcode: &u16) -> PcState {
        self.registers[0xF] = self.registers[((opcode & 0x0F00) >> 8) as usize] & 0x1;
        self.registers[(opcode & 0x0F00 >> 8) as usize] >>= 1;
        PcState::Next
    }
    // 0x8XY7: Sets VX to VY minus VX. VF is set to 0 when there's
    // a borrow, and 1 when there isn't. 8000. 0007
    fn op_8xy7(&mut self, opcode: &u16) -> PcState {
        if(self.registers[((opcode & 0x0F00) >> 8) as usize] > self.registers[((opcode & 0x00F0) >> 4) as usize]) {
            self.registers[0xF] = 0; // there is a borrow
        }
        else {
            self.registers[0xF] = 1;
        }
        self.registers[((opcode & 0x00F0) >> 4) as usize] -= self.registers[((opcode & 0x0F00) >> 8) as usize];
        PcState::Next
    }
    // 0x8XYE: Shifts VX left by one. VF is set to the value of
    // the most significant bit of VX before the shift. 8000. 000e 
    fn op_8xye(&mut self, opcode: &u16) -> PcState {
        self.registers[0xF] = self.registers[((opcode & 0x0F00) >> 8) as usize] & 0x1;
        self.registers[(opcode & 0x0F00 >> 8) as usize] <<= 1;
        PcState::Next
    }
    // 9XY0 - Skips the next instruction if VX doesn't equal VY. 9000
    fn op_9xy0(&self, opcode: &u16) -> PcState {
        if(self.registers[(opcode & 0x0F00 >> 8) as usize] != self.registers[((opcode & 0x00F0) >> 4) as usize]) {
            return PcState::Skip;
        }
        else {
            return PcState::Next;
        }
        
    }
    // ANNN - Sets I to the address NNN. a000
    fn op_annn(&mut self, opcode: &u16) -> PcState {
        self.index = opcode & 0x0FFF;
        PcState::Next
    }
    // BNNN - Jumps to the address NNN plus V0. b000
    fn op_bnnn(&mut self, opcode: &u16) -> PcState {
        PcState::Jump((((opcode) & (0x0FFF)) as u16 + self.registers[0] as u16) as usize)
    }
    // CXNN - Sets VX to a random number, masked by NN. c000
    fn op_cxnn(&mut self, opcode: &u16) -> PcState {
        //watch out for overflow
        self.registers[(opcode & 0x0F00 >> 8) as usize] = (opcode & 0x00FF) as u8;
        PcState::Next
    }
    // DXYN: Draws a sprite at coordinate (VX, VY) that has a width of 8
    // pixels and a height of N pixels.
    // Each row of 8 pixels is read as bit-coded starting from memory
    // location I;
    // I value doesn't change after the execution of this instruction.
    // VF is set to 1 if any screen pixels are flipped from set to unset
    // when the sprite is drawn, and to 0 if that doesn't happen. d000
    fn op_dxyn(&mut self, opcode: &u16) -> PcState {
        let mut x = self.registers[(opcode & 0x0F00 >> 8) as usize];
        let mut y = self.registers[((opcode & 0x00F0) >> 4) as usize];
        let nibble = (opcode) & (0x000F);
        let mut counter = self.index;

        if (x > 64) {
            x = x-64;
        }
        if (y > 32) {
            y = y-32;
        }

        let sprite_height_processed = 0;
        
        let mut offsetX = 0;
        //the y offset is "i" the only reason I have an x offset is bc I'm too lazy to come up with a different way to do it. (the offsetX is basically an index)
        for i in 0..nibble {
            
            

            let byte = self.memory[(counter) as usize];
            counter += 1 as u16;

            println!("{}", byte);

            if i > 32 {
                break
            }
            

            for j in 0..7 {
                
                if offsetX > 64 {
                    break
                }

                if byte & (128 >> offsetX) > 0 {
                    //hopefully casting a u16 to a u8 does not cause some obscure problem that makes no sense
                    self.io.draw_pixel(offsetX + x, i as u8 + y);
                }

                offsetX += 1;
            }
            offsetX = 0;
        }
        PcState::Next
    }
    // EX__. e000. yet another thing that is read by the reader but has no function instead used for other stuff

    // EX9E - Skips the next instruction if the key stored
    // in VX is pressed. e000. 009e
    fn op_ex9e(&self, opcode: &u16) -> PcState {
        if self.io.keys[self.registers[(opcode & 0x0F00 >> 8) as usize] as usize] {
            return PcState::Skip
        } 
        PcState::Next
    }
    // EXA1 - Skips the next instruction if the key stored
    // in VX isn't pressed.
    fn op_exa1(&self, opcode: &u16) -> PcState {
        if !self.io.keys[self.registers[(opcode & 0x0F00 >> 8) as usize] as usize] {
            return PcState::Skip
        } 
        PcState::Next
    }
    // FX__. f000. here we go agein refer to EX__.

    // FX07 - Sets VX to the value of the delay timer. f000. 0007
    fn op_fx07(&mut self, opcode: &u16) -> PcState {
        self.registers[(opcode & 0x0F00 >> 8) as usize] = self.delay_timer;
        PcState::Next
    }
    // FX0A - A key press is awaited, and then stored in VX. f000. 000a
    fn op_fx0a(&mut self, opcode: &u16) -> PcState {
        let mut key_pressed = false;
        while !key_pressed{
            for i in 0..self.io.keys.len() {
                if self.io.keys[i] == true {
                    self.registers[(opcode & 0x0F00 >> 8) as usize] = i as u8;
                    key_pressed = true;
                }
            }
        }
        PcState::Next
    }
    // FX15 - Sets the delay timer to VX. f000. 0015
    fn op_fx15(&mut self, opcode: &u16) -> PcState {
        self.delay_timer = self.registers[((opcode & 0x0F00) >> 8) as usize];
        PcState::Next
    }
    // FX18 - Sets the sound timer to VX. f000. 0018
    fn op_fx18(&mut self, opcode: &u16) -> PcState {
        self.sound_timer = self.registers[((opcode & 0x0F00) >> 8) as usize];
        PcState::Next
    }
    // FX1E - Adds VX to I. f000. 001e
    fn op_fx1e(&mut self, opcode: &u16) -> PcState {
        self.index = self.registers[(opcode & 0x0F00 >> 8) as usize] as u16 + self.index;
        PcState::Next
    }
    // FX29 - Sets I to the location of the sprite for the
    // character in VX. Characters 0-F (in hexadecimal) are
    // represented by a 4x5 font. f000. 0029
    fn op_fx29(&mut self, opcode: &u16) -> PcState {
        self.index = (self.registers[((opcode & 0x0F00) >> 8) as usize] * 0x5) as u16; 
        PcState::Next
    }
    // FX33 - Stores the Binary-coded bcd representation of VX
    // at the addresses I, I plus 1, and I plus 2. f000. 0033
    fn op_fx33(&mut self, opcode: &u16) -> PcState {
        //look at stuff with val if this instruction fails
        let mut val = self.registers[((opcode & 0x0F00) >> 8) as usize];
        
        for i in 3..1 {
            self.registers[(self.index + i - 1) as usize] = val % 10;
            val = val / 10;
        }
        PcState::Next
    }
    // FX55 - Stores  V0 toVX in memory starting at address I. f000. 0055
    fn op_fx55(&mut self, opcode: &u16) -> PcState {
        for i in 0..(opcode & 0x0F00 >> 8) {
            self.memory[(self.index + i) as usize] = self.registers[(i) as usize];
        }
        self.index += ((opcode & 0x0F00) >> 8) + 1;
        PcState::Next
    }
    // FX65 - I do not know what this does refer to https://github.com/JamesGriffin/CHIP-8-Emulator/blob/master/src/chip8.cpp line 460 for implmentation details. f000. 0065
    fn op_fx65(&mut self, opcode: &u16) -> PcState {
        for i in 0..(opcode & 0x0F00 >> 8) {
            self.registers[(i) as usize] = self.memory[(self.index + i) as usize] ;
        }
        self.index += ((opcode & 0x0F00) >> 8) + 1;
        PcState::Next
    }
}
