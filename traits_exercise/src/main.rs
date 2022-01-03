struct Dwarf {
    name: String,
}

struct Human {
    name: String,
}

struct HalfOrc {
    name: String,
}

struct Elf {
    name: String,
}

struct HalfElf {
    name: String,
}

pub trait Constitution {
    fn constitution_bonus(&self) -> u8 {
        0
    }
}

impl Constitution for HalfOrc {
    fn constitution_bonus(&self) -> u8 {
        1
    }
}

impl Constitution for Dwarf {
    fn constitution_bonus(&self) -> u8 {
        2
    }
}

impl Constitution for Human {}

impl Constitution for Elf {}

pub trait Elvish {}
impl Elvish for Elf {}
impl Elvish for &HalfElf {}

pub fn speak_elvish<T: Elvish>(_character: T) -> String {
    String::from("yes")
}

fn main() {
    let john = Human {
        name: String::from("John"),
    };
    john.constitution_bonus();

    let steffan = HalfOrc {
        name: String::from("Steffan"),
    };
    steffan.constitution_bonus();

    let markus = Dwarf {
        name: String::from("Markus"),
    };
    markus.constitution_bonus();

    let liam = Elf {
        name: String::from("Liam"),
    };
    liam.constitution_bonus();
    &liam.name;
    speak_elvish(liam);

    let straga = HalfElf {
        name: String::from("Straga"),
    };
    let can_speak = speak_elvish(&straga);
    let straga_name = &straga;
}
