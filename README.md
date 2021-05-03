# Deref with Traits

This repo is a question to my tutor, the awesome
[Keruspe](https://github.com/Keruspe).

## The problem

    git checkout eea1c7d

I wrote two traits, and I want one to "inherit" the other.
In this example the trait `Running` should "inherit" the trait `Walking`,
meaning that anything that can `run()` should be able to `walk()`.

```rust
// lib.rs
pub trait Walking {
    fn walk(&self);
}

pub trait Running{
    fn run(&self);
    fn walk(&self);
}
```

But I don't want to reimplement the `walk()` method for everything that implements `Running`!

What I want to do is something like:

```rust
impl Contest {
    // this function takes an arguments that satisfies Walking…
    pub fn welcome_a_walker(&self, walker: &dyn Walking) {
        walker.walk();
    }

    pub fn start_a_race(
        &self,
        runner: &dyn Running,
    ) {
        // … and should be called on arguments that satisfy Running!
        self.welcome_a_walker(runner);
        runner.run();
    }

}
```

Alas…

    22 |  self.welcome_a_walker(runner);
       |                        ^^^^^^ expected trait `Walking`, found trait `Running`
       |

My tutor told me to look at the
[`Deref` trait](https://doc.rust-lang.org/std/ops/trait.Deref.html).
Using that trait, I should be able to do:

```rust
//structs.rs
pub struct Walker { pub name: String }

impl Walking for Walker {
    fn walk(&self) { /* whatever*/  }
}

pub struct Runner {
    pub walker_with_name: Walker, // Runner contains an inner Walker
    pub sprint_time: f32,
}

impl Running for Runner {
    fn run(&self) { /* whatever */ }

    fn walk(&self){
        // call the walk method on the dereferenced inner Walker
        self.deref().walk()
    }
}

// The magic happens here.
impl Deref for Runner {
    type Target = Walker;
    fn deref(&self) -> &Self::Target {
        &self.walker_with_name
    }
}
```

Still, doesn't work. But the tutor knows better:

    git checkout 3364ee6

It is a beauty:

```rust
// lib.rs
pub trait Walking {
    fn walk(&self);
}

pub trait Running: Walking {
    fn run(&self);
}

// some nice black magic here.
impl<T: Deref<Target = U>, U: Walking> Walking for T {
    fn walk(&self) {
        self.deref().walk()
    }
}
```

Expressed in english, the black magic reads like this:

> Dear compiler,  
> please implement the trait `Walking` for a generic type `T`
>
> This type should itself implement the `Deref` trait with a target that implements
> the `Walking` trait.

It looks obvious now, but believe me, I have smashed my brain against my keyboard a number of times
but I could never have found out myself.
