use std::path::Path;
use std::fs::File;

pub fn copy(from: &String, to: &String) {
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    
    let file = match File::open(&from) {
        Err(e) => panic!("couldn't open {}: {}", from_path.display(), e),
        Ok(file) => file,
    };
    
        
}
