use bit_vec::BitVec;

struct Multiple {
  multiplicator: u32,
  index: u32,
  end: u32,
}

impl Iterator for Multiple {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let next = self.index * self.multiplicator;
        if next >= self.end {
            return None
        }

        self.index += 1;
        return Some(next)
    }
}

fn multiplication_table_of(multiplicator: u32, to: u32) -> Multiple {
    // index starts at 2 because 1 is always a prime number
    Multiple { multiplicator: multiplicator, index: 2, end: to }
}

fn main() {
  let max_number = 2_000_000 as usize;
  let max_number_sqrt = (max_number as f64).sqrt() as u32 + 1;
  let mut sieve = BitVec::from_elem(max_number, false);

  for i in 2..max_number_sqrt {
    if ! sieve[i as usize] {
        for j in multiplication_table_of(i, max_number as u32) {
            sieve.set(j as usize, true);
        }
    }
  }

  println!("Le primes:");
  for i in 2..max_number {
    if ! sieve[i] {
      print!("{}, ", i);
    }
  }
  print!("\n")
}
