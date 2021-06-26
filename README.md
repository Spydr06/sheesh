# sheesh
!! This shell is far away from being somehow usable or in a working condition !!
A small and fast linux shell written in Rust, inspired by [hilbish](https://github.com/Rosettea/Hilbish) and [fish](https://github.com/fish-shell/fish-shell).

Note, that this shell will not be POSIX compatible.

## Installation

To install sheesh, first download this repository and enter it:
```bash
git clone https://github.com/spydr06/sheesh.git
cd ./sheesh
```
Then compile and install it using cargo:
```bash
cargo install --path .
```
## Syntax

At the moment, I'm not very sure about how the syntax will be, although I opt for [Ruby](https://www.ruby-lang.org/en/) or [Crystal](https://crystal-lang.org/) like syntax since its easy to read, write and (most importantly) parse.
So (hopefully) the syntax will look something like this:

```ruby
#!/bin/sheesh
# import another script
require 'foo.sh'

# set a useful alias
alias ls = "ls -al"

# define some functions
def foo
    echo "Hello World!"
    
    $value = true
    if $value
        echo "{$value} will always be true"
    end
end

foo()
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
sheesh is licensed under the [MIT license](https://mit-license.org/).
