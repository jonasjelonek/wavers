
pub fn map_u8_to_i16(val: u8) -> i16 {
    let moved = (val as i16) - 128_i16;
    if moved < 0 {
        (((moved as f32) / (i8::MIN as f32)) * (i16::MIN as f32)) as i16
    } else {
        (((moved as f32) / (i8::MAX as f32)) * (i16::MAX as f32)) as i16
    }
}

pub fn map_u8_to_i32(val: u8) -> i32 {
    let moved = (val as i32) - 128_i32;
    match moved < 0 {
        true => (((moved as f64) / (i8::MIN as f64)) * (i32::MIN as f64)) as i32,
        false => (((moved as f64) / (i8::MAX as f64)) * (i32::MAX as f64)) as i32
    }
}

pub fn map_u8_to_i64(val: u8) -> i64 {
    let moved = (val as i64) - 128_i64;
    match moved < 0 {
        true => (((moved as f64) / (i8::MIN as f64)) * (i64::MIN as f64)) as i64,
        false => (((moved as f64) / (i8::MAX as f64)) * (i64::MAX as f64)) as i64
    }
}

pub fn map_u8_to_f32(val: u8) -> f32 {
    let moved = (val as i32) - 128_i32;
    match moved < 0 {
        true => (moved as f32) / (i8::MIN as f32).abs(),
        false => (moved as f32) / (i8::MAX as f32)
    }
}

pub fn map_u8_to_f64(val: u8) -> f64 {
    let moved = (val as i32) - 128_i32;
    match moved < 0 {
        true => (moved as f64) / (i8::MIN as f64).abs(),
        false => (moved as f64) / (i8::MAX as f64)
    }
}

pub fn map_i16_to_i32(val: i16) -> i32 {
    match val < 0 {
        true => (((val as f32) / (i16::MIN as f32)) * (i32::MIN as f32)) as i32,
        false => (((val as f32) / (i16::MAX as f32)) * (i32::MAX as f32)) as i32
    }
}

pub fn map_i16_to_i64(val: i16) -> i64 {
    match val < 0 {
        true => (((val as f32) / (i16::MIN as f32)) * (i64::MIN as f32)) as i64,
        false => (((val as f32) / (i16::MAX as f32)) * (i64::MAX as f32)) as i64
    }
}

pub fn map_i16_to_f32(val: i16) -> f32 {
    match val < 0 {
        true => (val as f32) / (i16::MIN as f32).abs(),
        false => (val as f32) / (i16::MAX as f32)
    }
}

pub fn map_i16_to_f64(val: i16) -> f64 {
    match val < 0 {
        true => (val as f64) / (i16::MIN as f64).abs(),
        false => (val as f64) / (i16::MAX as f64)
    }
}

pub fn map_i24_to_i32(val: i32) -> i32 {
    let i24_max = 8388607;
    let i24_min = -8388608;
    match val < 0 {
        true => (((val as f64) / (i24_min as f64)) * (i32::MIN as f64)) as i32,
        false => (((val as f64) / (i24_max as f64)) * (i32::MAX as f64)) as i32
    }
}

pub fn map_i24_to_i64(val: i32) -> i64 {
    let i24_max = 8388607;
    let i24_min = -8388608;
    match val < 0 {
        true => (((val as f32) / (i24_min as f32)) * (i64::MIN as f32)) as i64,
        false => (((val as f32) / (i24_max as f32)) * (i64::MAX as f32)) as i64
    }
}

pub fn map_i24_to_f32(val: i32) -> f32 {
    let i24_max = 8388607;
    let i24_min = -8388608;
    match val < 0 {
        true => (val as f32) / (i24_min as f32).abs(),
        false => (val as f32) / (i24_max as f32)
    }
}

pub fn map_i24_to_f64(val: i32) -> f64 {
    let i24_max = 8388607;
    let i24_min = -8388608;
    match val < 0 {
        true => (val as f64) / (i24_min as f64).abs(),
        false => (val as f64) / (i24_max as f64)
    }
}

pub fn map_i32_to_i64(val: i32) -> i64 {
    match val < 0 {
        true => (((val as f32) / (i32::MIN as f32)) * (i64::MIN as f32)) as i64,
        false => (((val as f32) / (i32::MAX as f32)) * (i64::MAX as f32)) as i64
    }
}

pub fn map_i32_to_f32(val: i32) -> f32 {
    match val < 0 {
        true => (val as f32) / (i32::MIN as f32).abs(),
        false => (val as f32) / (i32::MAX as f32)
    }
}

pub fn map_i32_to_f64(val: i32) -> f64 {
    match val < 0 {
        true => (val as f64) / (i32::MIN as f64).abs(),
        false => (val as f64) / (i32::MAX as f64)
    }
}

pub fn map_i64_to_f32(val: i64) -> f32 {
    match val < 0 {
        true => ((val as f64) / (i64::MIN as f64).abs()) as f32,
        false => ((val as f64) / (i64::MAX as f64)) as f32
    }
}

pub fn map_i64_to_f64(val: i64) -> f64 {
    match val < 0 {
        true => (val as f64) / (i64::MIN as f64).abs(),
        false => (val as f64) / (i64::MAX as f64)
    }
}