# 1.0.0: Initial release
Date: 02.06.2025

Public release

# 1.0.1: Readme fixes
Date: 02.06.2025

Fix markdown rendering on crates.io

# 1.0.2: Readme fixes
Date: 02.06.2025

Fix markdown rendering on crates.io

# 1.0.3: Add changelog
Date: 02.06.2025

Add Changelog.md

# 1.1.0: Broaden `ErrorMessage` construction functions
Date: 07.06.2025

Previously `ErrorMessage::new` required a `String`.  
Now, it only requires something that has the `ToString` trait.  
Same for `ErrorMessage::err` and `ErrorMessage::with_context`
