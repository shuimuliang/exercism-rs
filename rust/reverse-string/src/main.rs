fn main() {
   let s = "子猫";
   let svec = String::from(s);
   let mut res = String::new();
   for t in svec.chars().rev()  {
      res.push(t);
   }
   println!("{}", res);
}