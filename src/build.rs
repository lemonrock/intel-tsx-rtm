// This file is part of intel-tsx-rtm. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT. No part of intel-tsx-rtm, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-tsx-rtm. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT.


extern crate rust_c;


use ::std::env;


fn main()
{
	let absolute_home_folder_path = env::var("CARGO_MANIFEST_DIR").unwrap();
	
	compile_embedded_c_code(&absolute_home_folder_path);
}

fn compile_embedded_c_code(absolute_home_folder_path: &str)
{
	match env::var("CROSS_COMPILE")
	{
		Ok(_) => (),
		Err(_) =>
		{
			println!("cargo:warning=Please specify CROSS_COMPILE=x86_64-linux-musl- cargo build --target=x86_64-unknown-linux-musl as the gcc crate incorrectly looks for musl-gcc");
			return;
		}
	};
	
	let include_folder_path = format!("{}/lib/tsx-tools/include", absolute_home_folder_path.to_owned());
	
	let path = format!("{}/src/lib.rs", absolute_home_folder_path);
	rust_c::build(path, "intel-tsx-rtm_c", |gcc_config|
	{
		gcc_config.flag("-Werror");
		gcc_config.define("_GNU_SOURCE", None);
		gcc_config.define("_BSD_SOURCE", None);
		gcc_config.flag(&format!("-isystem{}", include_folder_path)); // can't use .include() as warnings then occur in system headers
		gcc_config.flag("--mrtm");
		gcc_config.opt_level(3);
	});
}
