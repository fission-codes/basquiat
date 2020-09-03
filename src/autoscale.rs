use crate::cfg_parser::Config;

fn autoscale_config(config: Config, spread_val: i32) -> Vec<Config>{
    let scale_delta: f32 = 1.0 / (spread_val as f32);
    let mut configs: Vec<Config>= Vec::new();
    for n in 1..spread_val{
        let scalar = ((n as f32)*scale_delta).powf(0.5); // The scales are linearly spread out in terms of number of pixels, hence we take the square root here
        let cur_config = Config{
            dimensions: config.dimensions*scalar,
            ..config
        };
        configs.push(cur_config);
    }
    return configs;
}

pub fn autoscale(configs: &Vec<Config>, spread_val: i32) -> Vec<Config>{
    if configs.len() != 0{
        return autoscale_config(configs[0], spread_val)
    } else {
        return Vec::new()
    }
}