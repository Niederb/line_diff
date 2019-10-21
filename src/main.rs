extern crate difference;

fn main() {
	use difference::Changeset;
	use difference::Difference;

	let s1 = "set(OpenCV_COMPUTE_CAPABILITIES -gencode;arch=compute_61,code=sm_61;-gencode;arch=compute_75,code=sm_75;-gencode;arch=compute_61,code=compute_61;-gencode;arch=compute_75,code=compute_75)";
	let s2 = "set(OpenCV_COMPUTE_CAPABILITIES -gencode;arch=compute_52,code=sm_52;-gencode;arch=compute_61,code=sm_61;-gencode;arch=compute_70,code=sm_70;-gencode;arch=compute_75,code=sm_75;-gencode;arch=compute_52,code=compute_52;-gencode;arch=compute_61,code=compute_61;-gencode;arch=compute_70,code=compute_70;-gencode;arch=compute_75,code=compute_75)";

	let separator = ";";
	let s1 = s1.replace(separator, "\n");
	let s2 = s2.replace(separator, "\n");

	let separator = ",";
	let s1 = s1.replace(separator, "\n");
	let s2 = s2.replace(separator, "\n");

	let changeset = Changeset::new(&s1, &s2, "\n");
	println!("Newline");
	println!("{}", changeset);



}
