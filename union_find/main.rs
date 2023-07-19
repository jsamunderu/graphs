// Converted to rust from Java
// From Algorithms. Forth edition. Sedgewick & Wayne, p 216 

mod graphs {
    pub struct UnionFind {
        _count: usize,
        _id: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            let mut id = Vec::<usize>::with_capacity(n);
            for i in 0..n {
                id.push(i);
            }
            Self {
                _count: n,
                _id: id,
            }
        }

        pub fn count(&mut self) -> usize {
            self._count
        }

        pub fn connected(&mut self, p: usize, q: usize) -> bool {
            self.find(p) == self.find(q)
        }

        pub fn find(&mut self, p: usize) -> usize {
            self._id[p]
        }

        pub fn union(&mut self, p: usize, q: usize) {
            let i = self.find(p);
            let j = self.find(q);
            if i == j {
                return
            }
            self._id[i] = j;
            self._count -= 1;
        }
    }
}

fn main() {
    let mut sizeText = String::new();
    std::io::stdin().read_line(&mut sizeText).expect("failed to read from stdin");
    let trimmed = sizeText.trim();
    let mut n: usize = 0;
    match trimmed.parse::<usize>() {
        Ok(result) => {
            n = result;
        },
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    let mut unionFind = graphs::UnionFind::new(n);
    loop {
        let mut inputText = String::new();
        std::io::stdin().read_line(&mut inputText).expect("failed to read from stdin");
        let result = inputText.split_whitespace().map(|x| x.parse::<usize>()).collect::<Result<Vec<usize>, _>>();

        let mut p: usize = 0;
        let mut q: usize = 0;
        match result {
            Ok(values) => {
                p = values[0];
                q = values[1];
            },
            Err(..) => {
                println!("this was not an integer: {}", trimmed);
                break;
            },
        };

        if unionFind.connected(p, q) {
            continue;
        }
        unionFind.union(p, q);
        print!("{} {}", p, q);
    }
    println!("{} components", unionFind.count());
}