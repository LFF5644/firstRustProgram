fn main(){
	let start_num=0;
	let text=format!("Es Wird bei {} gestartet",start_num);
	print(&text);
	for i in start_num..10{
		print(&i.to_string());
	}
}

fn print(text: &str){
	println!("{}",text);
}