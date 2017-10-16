/*!
Changing the default logging format.

Before running this example, try setting the `MY_LOG_LEVEL` environment variable to `info`:

```no_run,shell
$ export MY_LOG_LEVEL = 'info'
```

If you want to control the logging output completely, see the `custom_logger` example.
*/

#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;

fn main() {
    let mut builder = env_logger::Builder::new();

    builder.format(|buf, record| {
        writeln!(buf, "My formatted log: {}", record.args())
    });

    if let Ok(s) = env::var("MY_LOG_LEVEL") {
        builder.parse(&s);
    }

    builder.init();

    info!("a log from `MyLogger`");
}
