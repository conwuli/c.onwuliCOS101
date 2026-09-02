fn main () {
	println!("Sales Record for P.M Okeke and Sons Ltd");
	let t_amt:i32 = 450_000; // t_amt is amount of toshiba sold
	let m_amt:i32  = 1_500_000;// m_amt is amount of mac
	let hp_amt:i32  = 750_000; // amount of hp sold
	let d_amt:i32  = 2_850_000; // amount of dell sold
	let a_amt:i32  = 250_000; //amount of acer sold
	let t_qty:i32  = 2; // quantity of toshiba
	let m_qty:i32  = 1; // "         " mac
	let hp_qty:i32  = 3; // "     "    hp
	let d_qty:i32  = 3; // "      " dell
	let a_qty:i32  = 1; // "    " acer
	let s:i32 = t_amt + m_amt + hp_amt + d_amt + a_amt; // sum of sales
	let a:i32 = s / (t_qty + m_qty + hp_qty + d_qty + a_qty); // average of sales
	println!("The sum of the sales record is {}",s );
	println!("The average of the sales record is {}",a );


} 