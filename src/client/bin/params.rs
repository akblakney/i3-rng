
//pub fn convert_int(args: &Vec<String>, index: usize) -> usize {
//  let ret = args[index+1].parse::<usize>();
//  let ret = match ret {
//    Ok(x) => x,
//    Err(_) => {
//      eprintln!("must give integer argument to -n flag");
//      std::process::exit(1);
//  }
//  return Ok(ret);


pub fn get_param<'a>(args: &'a Vec<String>, flag: &'a str) -> Result<&'a str, String> {
  if args.contains(&flag.to_string()) {
    let index = args.iter().position(|x| x == flag).unwrap();
    if args.len () < index + 2 {
      eprintln!("must give argument for {} flag", flag);
      std::process::exit(1);
    }
    return Ok(&args[index + 1]);
  }
  return Err("no flag".to_string());
}
