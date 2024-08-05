use rand::Rng;

pub fn random<T>(min: T, max: T) -> T 
where
    T: rand::distributions::uniform::SampleUniform + Copy + std::cmp::PartialOrd,
{
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}