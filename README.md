# Snake build with Bevy 0.11

After fumbling around with Godot, Unity and Bevy I found that I struggled most with game development related stuff like assets and worlds. In order to make some progress I decided to first start with a very simple game. Thats when I landed on Snake. Since this was a fundamentally simple game I decided to implement it in my "dream" game engine which was bevy which prior to this I had not chosen because of.

## What was my goal?

Write a basic snake game like the one from a Nokia 3410.

## Why Bevy?

I think rust can be good for preventing issues early. Also i liked to try out rust. Lastly Bevy's highly parallel architecture spoke to me.

## What did I like?

I liked the highly parallel architecture by default.
I also like the idea of ECS. I enjoyed working with Rust a lot but aren't that productive with it.

I ended up not implementing sound effects and scoring as i don't think i would have learned a lot extra from it and the documentation on audio especially was lacking

## What did I not like

* ECS takes away all compile time checking. For me this defeats the point of rust completely. You only find out whether components are not there on runtime which i think is bad.
* I find i am duplicating some stuff in ECS and cannot make use of rust structures to create good component composition.
* I also find the documentation very lacking.
* I dabbled with deploying on ios and android and it seems the project isn't there yet.
* It seems like the physics engine is not great at the moment, there is bevy xbpd but no character controllers exist for it.
* No editor also makes the feedback loop much slower.
* There are many breaking changes in the bevy ecosystem and plugins are often unmaintained
* I am concerned with rust's future as a programming language as it seems that its very overhyped but other languages offer similar features and are much more productive 

## Conclusions

I will now build games in Godot instead, which has almost all the above missing features. I don't like GDScript and C# nearly as much but they do support hot reloading. Also there is a rust binding project for when these languages don't work out for me. Maybe in a couple years i'll have another look at bevy.

## Some later comments from 2024
I feel Rust doesn't really offer advantages in solo projects where you know whats going on. I also think most projects don't need the extreme safety rust provides. I feel rust is good for stuff like embedded where the consequences of memory leaks and runtime errors are dire. For most applications with a kernel i would almost always use golang, or when gc is a dealbreaker use swift. For Godot i have decided to use SwiftGodot as the go bindings are not very mature. I have also since moved to a more pragmatic approach where i care less about the technology itself but more to the environment and what gets the project finished in an acceptable manner. Lastly i think its quite easy to make unmaintainable code in rust as its easy to start wrapping everything in mutexes to make the compiler shut up. Golang does not have this issue as it plain doesn't allow you to use complex structures keeping the code maintainable and readable
