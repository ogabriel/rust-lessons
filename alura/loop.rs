pub fn main() {
   let multiplicador:u8 = 5;

   let mut contador:u8 = 0;

   println!("--while--");

   while contador < 10 {
       contador += 1;

       if contador == 5 { continue; }

       println!("{} X {} = {}", multiplicador, contador, multiplicador * contador);
   }

   println!("--loop--");

   contador = 0;

   loop {
       contador += 1;
       println!("{} X {} = {}", multiplicador, contador, multiplicador * contador);

       if contador >= 10 { break; }
    }

   println!("--for--");

   for i in 1..=10 {
       println!("{} X {} = {}", multiplicador, i, multiplicador * i);
   }
}
