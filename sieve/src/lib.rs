pub fn primes_up_to(top: i32) -> Vec<i32> {
	let mut queue: Vec<i32> = (2..top+1).collect();
	let mut i:usize = 0;
	while i < queue.len(){
		queue = queue.iter()
			.filter(|x| {
				(**x <= queue[i]) || ( (**x % queue[i]) != 0)
			})
			.cloned()
			.collect::<Vec<i32>>();
		i += 1;
	}
	queue
}