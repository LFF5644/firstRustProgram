
fn main(){
	//print("Nummer startet bei "+text);
	for i in 0..10{
		print(&i.to_string());
	}
}

fn print(text: &str){
	println!("{}",text);
}