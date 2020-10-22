/*
 * The contents of this file are subject to the terms of the
 * Common Development and Distribution License, Version 1.0 only
 * (the "License").  You may not use this file except in compliance
 * with the License.
 *
 * See the file LICENSE in this distribution for details.
 * A copy of the CDDL is also available via the Internet at
 * http://www.opensource.org/licenses/cddl1.txt
 *
 * When distributing Covered Code, include this CDDL HEADER in each
 * file and include the contents of the LICENSE file from this
 * distribution.
 */

use chrono::prelude::*;
use config::{Config, File};
use egg_mode::{KeyPair, Token, tweet::DraftTweet};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Konfigurationsdatei:
	let mut settings = Config::default();
	settings.merge(File::with_name("kuckuck")).unwrap();

	// Twitterlogin:
	let consumer_key = settings.get::<String>("consumer_key")?;
	let consumer_secret = settings.get::<String>("consumer_secret")?;
	let access_key = settings.get::<String>("access_key")?;
	let access_secret = settings.get::<String>("access_secret")?;

	let consumer_token = KeyPair::new(consumer_key, consumer_secret);
	let access_token = KeyPair::new(access_key, access_secret);
	let token = Token::Access {
		consumer: consumer_token,
		access: access_token,
	};

	// Aktuelle Stunde:
	let (_, stunde) = Local::now().hour12();
	let stunde_usize = stunde as usize;
	let kuckuck = "Kuckuck! ".repeat(stunde_usize);

	// Und ab damit:
	let tweet = DraftTweet::new(kuckuck);
	tweet.send(&token).await?;

	Ok(())
}
