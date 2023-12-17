use crate::sstr::Sstr;
use rand::distributions::Uniform;


#[derive(Debug)]
pub enum Kind<'a, 'b>{
    None,
    Further(Holder<'a, 'b>),
    End(Sstr<'a>, u16),
}

pub struct BumbCrap<'a, 'b>{
    next_vals: &'b mut[[Kind<'a, 'b>;27]]
}

impl<'a, 'b> BumbCrap<'a, 'b>{
    pub fn new(vec: &'b mut Vec<[Kind<'a, 'b>;27]>) -> Self{
        const CAPACITY: usize = 1_000_000 *10;
        *vec = Vec::with_capacity(CAPACITY);
        vec.resize_with(CAPACITY, ||std::array::from_fn(|_|Kind::None));
        Self { next_vals: vec.as_mut_slice() }   
    }

    pub fn next(&mut self) -> &'b mut [Kind<'a, 'b>;27]  {
        // the actual safe way of doing this 
        // let slice = std::mem::replace(&mut self.next_vals, &mut []);
        // let (head, tail) = slice.split_first_mut().unwrap();
        // self.next_vals = tail;
        // head

        match self.next_vals{
            [next , other @ ..] => {
                // this is safe but the 'safe' way to do it adds a bit of overhead so we move
                self.next_vals = unsafe{std::mem::transmute(other)};
                return unsafe{std::mem::transmute(next)};
            }
            // pray you have enough slots/memory
            _ => unreachable!()
        }
    }
}




#[derive(Debug)]
pub struct Holder<'a, 'b>(&'b mut [Kind<'a, 'b>; 27]);

impl<'a, 'b> Holder<'a, 'b>{

    pub fn new(bumb: &mut BumbCrap<'a, 'b>) -> Self{
        Self(bumb.next())
    }

    #[inline(always)]
    fn add_t(&mut self, bumb: &mut BumbCrap<'a, 'b>, one: &'a str, one_count: u16, two: &'a str, mut level: usize){
        let mut myself = self;
        loop{
            let one_c = one.as_bytes().get(level).map(|v|*v as usize - b'A' as usize + 1).unwrap_or(0);
            let two_c = two.as_bytes().get(level).map(|v|*v as usize - b'A' as usize + 1).unwrap_or(0);
            if one_c != two_c{
                myself.0[one_c] = Kind::End(one.into(), one_count);
                myself.0[two_c] = Kind::End(two.into(), 1);
                return;
            }else{
                let thing = &mut myself.0[one_c];
                *thing = Kind::Further(Self::new(bumb));
                
                match thing{
                    Kind::Further(recurse) => {
                        level += 1;
                        myself = &mut *recurse;
                    }
                    _ => {unreachable!()}
                }
            }
        }

    }


    pub fn add(&mut self, bumb: &mut BumbCrap<'a, 'b>, data: &'a str, mut level: usize){
        let mut myself = self;
        loop{
            let char = data.as_bytes().get(level).map(|v|*v as usize  - b'A' as usize + 1).unwrap_or(0);
            // println!("{} {}", char as u8 as char, &data[level..1+level]);
            let val = &mut myself.0[char];
    
            match val{
                Kind::None => {
                    *val = Kind::End(data.into(), 1);
                    return;
                },
                Kind::Further(recurse) => {
                    level += 1;
                    myself = &mut*recurse;
                },
                Kind::End(other, num) => {
                    if other.as_str().eq(data){
                        *num += 1;
                    }else{ 
                        let other = other.as_str();
                        let num = *num;
                        let mut new = Self::new(bumb);
                        new.add_t(bumb, other, num, data, level + 1);
                        *val = Kind::Further(new);
                    }
                    return;
                },
            }
        }

    }
    pub fn inorder(&'a self, thing: &mut impl FnMut(&'a str)){
        for v in &*self.0{
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

    pub fn sort(&'a mut self, bumb: &mut BumbCrap<'a, 'b>, into: &mut Vec<&'a str>, vals: &[&'a str]) {
        for val in vals{
            self.add(bumb, val, 0)
        }
        self.inorder(&mut |val| {
            into.push(val)
        });
    }
}

pub fn run2(){
    const STR_LEN: usize = 100;
    const NUM_STRS: usize = 10_000_000;
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

    let mut bumb = Vec::new();
    let mut bumb = BumbCrap::new(&mut bumb);

    let now = std::time::Instant::now();
    let mut holder = Holder::new(&mut bumb);
    holder.sort(&mut bumb, &mut sorted, &vec);
    println!("mine: {} ms", now.elapsed().as_secs_f32() * 1000.0);


    let now = std::time::Instant::now();
    vec.sort();
    println!("rust: {} ms", now.elapsed().as_secs_f32() * 1000.0);

    assert_eq!(vec, sorted);
}