use rand::distributions::Uniform;

use crate::sstr::Sstr;


#[derive(Debug)]
enum Kind<'a>{
    None,
    Further(Box<Holder<'a>>),
    End(Sstr<'a>, u16),
}

#[derive(Debug)]
struct Holder<'a>([Kind<'a>; 27]);

impl<'a> Holder<'a>{

    pub fn new() -> Self{
        Self(std::array::from_fn(|_|Kind::None))
    }

    #[inline(always)]
    fn add_t(&mut self, one: &'a str, one_count: u16, two: &'a str, mut level: usize){
        let mut myself = self;
        loop{
            let one_c = one.as_bytes().get(level).map(|v|*v as usize - b'A' as usize + 1).unwrap_or(0);
            let two_c = two.as_bytes().get(level).map(|v|*v as usize - b'A' as usize + 1).unwrap_or(0);
            if one_c != two_c{
                myself.0[one_c] = Kind::End(one.into(), one_count);
                myself.0[two_c] = Kind::End(two.into(), 1);
                return;
            }else{

                let mut new = Kind::Further(Box::new(Self::new()));
                std::mem::swap(&mut myself.0[one_c], &mut new);
                std::mem::forget(new);
                
                match &mut myself.0[one_c]{
                    Kind::Further(recurse) => {
                        level += 1;
                        myself = &mut *recurse;
                    }
                    _ => {unreachable!()}
                }
            }
        }

    }


    pub fn add(&mut self, data: &'a str, mut level: usize){
        let mut myself = self;
        loop{
            let char = data.as_bytes().get(level).map(|v|*v as usize  - b'A' as usize + 1).unwrap_or(0);
            // println!("{} {}", char as u8 as char, &data[level..1+level]);
            let val = &mut myself.0[char];
    
            match val{
                Kind::None => {
                    let mut new =  Kind::End(data.into(), 1);
                    std::mem::swap(val, &mut new);
                    std::mem::forget(new);
                    // *val = Kind::End(data.into(), 1);
                    return;
                },
                Kind::Further(recurse) => {
                    // recurse.add(data, level + 1);
                    level += 1;
                    myself = &mut*recurse;
                },
                Kind::End(other, num) => {
                    if other.as_str().eq(data){
                        *num += 1;
                    }else{ 
                        let other = other.as_str();
                        let num = *num;

                        let mut new =  Kind::Further(Box::new(Self::new()));
                        std::mem::swap(val, &mut new);
                        std::mem::forget(new);
                        match val{
                            Kind::Further(recurse) => {
                                recurse.add_t(other, num, data, level + 1);
                            }
                            _ => {unreachable!()}
                        }
                        
                        // *val = Kind::Further(Box::new(new));
                    }
                    return;
                },
            }
        }

    }
    pub fn inorder(&'a self, thing: &mut impl FnMut(&'a str)){
        for v in &self.0{
            match v{
                Kind::None => {},
                Kind::Further(rabbit) => rabbit.inorder(thing),
                Kind::End(val, num) => {
                    for _ in 0..*num{
                        thing(val)
                    }
                },
            }
        }
    }

    pub fn sort(&'a mut self, into: &mut Vec<&'a str>, vals: &[&'a str]) {
        for val in vals{
            self.add(val, 0)
        }
        self.inorder(&mut |val| {
            into.push(val)
        });
    }
}

pub fn run1(){
    const STR_LEN: usize = 2;
    const NUM_STRS: usize = 1_00;
    let mut vec = Vec::with_capacity(NUM_STRS);

    let mut owned_vec = Vec::new();
    let mut rng = rand::thread_rng();
    owned_vec.resize_with(NUM_STRS, ||{
        let len = STR_LEN / 2 + rand::Rng::sample(&mut rng, Uniform::new(0,STR_LEN/2 + 1));
        let mut random = String::with_capacity(len);
        for _ in 0..len{
            random.push(rand::Rng::sample(&mut rng, Uniform::new(b'A', b'Z')) as char)
        }
        random
    });
    for item in &owned_vec{
        vec.push(item.as_str());
    }

    let mut sorted = Vec::with_capacity(vec.len());

    let now = std::time::Instant::now();
    let mut holder = Holder::new();
    holder.sort(&mut sorted, &vec);
    println!("mine: {} ms", now.elapsed().as_secs_f32() * 1000.0);


    let now = std::time::Instant::now();
    vec.sort();
    println!("rust: {} ms", now.elapsed().as_secs_f32() * 1000.0);

    assert_eq!(vec, sorted);

    println!("{:?}", sorted);
    println!("{:?}", vec);
    
}