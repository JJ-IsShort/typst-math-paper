
#import "template.typ": *
#import "@preview/physica:0.9.3": *

#show: project.with(
  title: "An Overview of Constraint Based Physics Solving", 
  authors: ("Jai Steinmetz",)
)

#heading("Introduction", bookmarked: true, depth: 1, outlined: true)
In this paper, I will try and explain how constraint based physics solvers work as well as trying to derive a few constraints and showcase their implementations. I decided on this as my topic because I have always like physics simulations but whenever I've tried to write my own I've had serious numerical stability issues and even when the simulations stayed stable for a bit they looked wrong. I recently found out why. I haven't found many resources on how to program a proper physics simulation, and even less easy to understand resources. I am hoping that putting together this paper and these simulations will both act as a resource for others and will help me understand these concepts better for future simulations I may want to write.

#pagebreak()
#heading("Background", bookmarked: true, depth: 1, outlined: true)
Physics solvers can be found everywhere, from Disney movies to our video games and from scientific research to computer simulations designed for no reason other than to look pretty (and get views on YouTube). In many of these applications, the quality of the physics simulation is extremely important. Disney wouldn't want to set their computers simulating the hair of characters for the day or two needed to get the quality they want and then find that the hair just jumped around weirdly or had numerical stability issues. Even in some games, the quality of the physics is important. For instance in the Poly Bridge series of games, the bridges that the player must build break if they experience too much force. There is a type of physics solver that often gets implemented that simply will not do for most of these applications.

#pagebreak()
#heading("Solver Types", bookmarked: true, depth: 1, outlined: true)
Impulse based physics solvers simply modify a velocity of acceleration parameter in a multitude of ways depending on the simulations requirements (gravity, collisions, etc) and then, as a final step, use an integrator like Verlet Integration or any of the RK4 Integrators (or if they don't care about quality, Euler Integration). These kinds of physics solvers often are of lower quality. For instance, the developers of the Poly Bridge game series wouldn't have gotten accurate force data if they used an Impulse Solver. With Impulse Solvers, multiple sources of force can conflict and result in odd behavior.

#linebreak()

Constraint Based Solvers first apply all the forces applied on an object, just like Impulse Solvers, except that when applying constraints like collision or distance constraints, in other words the rules which the simulation absolutely must not and can not break, it does those in a separate step. In this step, it takes information from each constraint present in the simulation and gathers them together. Then it uses the force applied by force generators and the constraint information to find the force that needs to be applied to keep those constraints valid as much as possible. By doing these steps separately and by using the unconstrained force to calculate a second corrective force, the constraints can be enforced in a much more rigorous way. You don't get issues like the forces of gravity in a stack of objects being so strong that they overcome the collision forces in the lower objects and push them into each other.

#pagebreak()
#heading("Simulations", bookmarked: true, depth: 1, outlined: true)
This paper is being put together by Typst as an alternative to LaTeX for formating and Raylib-rs compiled to WebAssembly for the interactivity and is being stitched together on a website using javascript. This allows me to create these sections with all sorts of content as well as sections that contain a (possibly interactive) simulation to show the equations in practice. For each of these simulations I will use Verlet Integration. The details of how it works aren't really needed for this paper. Its main advantages are how easy it is to implement, how incredibly numerically stable it is (especially when compared to Euler Integration which is what I was using before), how it doesn't lose energy with time, and how it is able to easily and as a property of the method itself conserve momentum without having to explicitly write it into other parts of the physics implementation. This will remove the need to write restitution factors into our simulation, simplifying it greatly. The previous simulation showcases a basic Impulse Solver. The gravity is being applied the same way it would be in most Constraint Solvers, but inter-object collision is missing entirely, and the two constraints that are built in (try clicking and dragging on it) are prone to the issues inherent in Impulse Solvers, such as several sources of movement fighting against each other and the constraints that your simulations aren't supposed to break get broken often. In this paper, those three constraints will be implemented, and more if there is time, in a much more rigorous way than implemented in the above simulation.

#pagebreak()
#heading("Derivation", bookmarked: true, depth: 1, outlined: true)
In this section I will go quickly over how we find the equation that gives us our corrective/constraint force $F_c$. First we should define a constraint. The idea behind a constraint is that it is a function that always evaluates too 0. In other words:
#align(center)[
  $C = 0$ (Position constraint)\
  $dot.basic(C) = 0$ (Velocity constraint)\
  $dot.double(C) = 0$ (Acceleration constraint)
]
This is the kind of notation you should get used to seeing. Typically a variable without and dot notation is referring to position, while one and two dots ($dot.basic(C)$ and $dot.double(C)$) refer to velocity and acceleration. So the goal is to find a vector which will bring the constraint back to 0. Let us first think about what the force $F_c$ is. It is a direction times a magnitude: $F_c = lambda N$. We can compute $N$, and this is how. $N = J^TT$ where $J$ is the Jacobian of the constraint. Let's break this down. The Jacobian just means that this matrix shows how much a change in the input simulation state values will change the value of the constraint. In other words:
#let C = "C"
#align(center)[
  $#partialderivative(C, "q") = jmat(C; "q"_x, "q"_y, "q"_z)$
]
Where $q$ is a list of the x, y and z positions of a particle. To help visualize this, let's imagine a 1D world with two particles. They have a constraint that tries to make them overlap. This means that $C(x_1,x_2) = x_1-x_2=0$ where $x_1$ and $x_2$ are the positions in 1D of the particles. If we find the Jacobian, we would get $vecrow(1,-1)$. This is a simplified example of finding the Jacobian.

#linebreak()
The next step is to explain a new variable, $W$ or the effective mass. $W = M^(-1)$ where $M$ is the mass matrix. And now to explain the mass matrix, which will change our understanding of the $q$ vector. It is a lot better to compute the force needed to correct all the objects all at once. As such, the entire state of the whole simulation gets used instead of the single particle that it has seemed like we are using. $M$ isn't just the mass of one particle, it's:\
#align(center)[
$M = mat(
    m_"1x", , , , ;
    , m_"1y", , , ;
    , , m_"1z", , ;
    , , , dots.down, ;
    , , , , m_"nx";
    , , , , , m_"ny";
    , , , , , , m_"nz";
)$
]
Here, each element is the inertia of a particle in one axis and the values are grouped with the other inertia values from the same object. This example is 3 dimensional. With this approach of  representing the whole state, $q$ becomes:
#align(center)[
  $q = vecrow(x_"1x",x_"1y",x_"1z",x_"2x",x_"2y",x_"2z",...,x_"nx",x_"ny",x_"nz")$
]
Where $n$ is the index of the particle. I'm going to leave the concept of the global state of the simulation behind for a bit since it was only needed to explain why mass is a matrix. Just understand that when I talk about the positions of the particles $q$, even if I show them as only 2 or 3 values depending on how many dimensions I use for the calculation, they are actually all the positions of all the particles.
// Add sources for this whole section to the slides and lecture notes. Notes are about the world vector thing. Slides are the individual equations

#linebreak()
Anyways, now we have $F_c=J^TT lambda$. We have the direction to apply the forces and with what intensity relative to others, however, we still need the magnitude multiplier. This is where I have to point out another caveat. This approach is how we can solve constraints in a computer. Technically, there is a different approach // Add sources to point to the slides. To show the other approach.
that gives an exact solution, but it is not the best approach for simulating for various reasons like not being as resistant to errors that would crop up in the imperfect world of the simulation. With that in mind, how should we best approximate the $F_c$ and really the $lambda$ in the world of computers? We can use $[J W J^TT]lambda =-dot.basic(J) dot.basic(q)-[J W]Q$ where $Q$ is similar to $q$ except that it is a force in each axis that was applied to each object this physics step before the correction force is applied. Since everything except $lambda$ is defined, the equation can be thought of as $A lambda=B$ or in other words, a linear system of equations. This is possible to solve in code using the conjugate gradient method. Implementing this method from the pseudocode and Julia code provided on the Wikipedia page //Add sources here to the wiki
was the hardest part of this whole project, though the main difficulty came from trying to use a pre written implementation instead of writing my own. Once I got to it it took me maybe a few hours. This means that if I needed to, armed with the knowledge of the path to take, I could probably get a full physics engine written in less than a week, and maybe you could too. There is still the $-dot.basic(J) dot.basic(q)$ term which is a bit scary, but luckily what the $dot.basic(J)$ means is $pdv(J,q,t)$ which will, in most cases, be 0. If we have time, we might look at an instance where it isn't 0, but it is also an edge case. This means that we get $[J W J^TT]lambda =-[J W]Q$. This, again, should be fairly easy to implement in code through matrix math libraries and the conjugate gradient solver.

#linebreak()
There is another element to this, where any errors need to be corrected for. This can be done easily with Baumgarte Stabilization// Add sources to the video
but it is not involving Calculus IV topics so we will brush over this.

#pagebreak()
#heading("Examples", bookmarked: true, depth: 1, outlined: true)
Now lets look at some constraints. We will re formulate the constraints that were there before, namely, the border and the cursor interaction constraints. First, lets look at what a conditional constraint even looks like, as the previous way to look at the constraints didn't allow them to only activate when a collision is happening. The solution is that we only apply the constraint if we detect a collision and otherwise ignore it. Lets only look at one axis. In this simulation, the floor is in the positive Y direction. To detect the collisions, we first make sure that the mouse isn't being pressed, as that is an element of the constraint, then we check if the position of an object is more or equal to the maximum extent of the screen in the Y axis. Now that we have a collision check, lets define $C$.
#align(center)[
  $C = 640 - a_y$
]
Where $a_y$ is the object's position on the Y axis. This then means that we need to find the Jacobian of this, and since it is an $RR => RR$ function, the Jacobian is actually just the derivative.
#align(center)[
  $dv(C,a_y) = -1$
]
This means that when our objects are colliding with the floor, value of the derivative in the Y axis is $-1$.
In practice, the Jacobian is multiplied by $0.5$ because that gives the constraint bounciness. Below this panel is a simulation with the screen edge constraint implemented to showcase the Jacobian previously calculated in practice and still using the old mouse following constraint.

#linebreak()
For the circular constraint, once again, we need to figure out when the constraint needs to be applied. It is only needed when the distance between the mouse position and the object is more than the following distance, which we can call $l$. We sill use a slightly different approach to finding the Jacobian here. This way is more generalized. We will find $dv(C, t)$ and use $dv(C,t) = J v$ where $v$ is a list of all the positions of the objects involved in this constraint. // Add sources to the other paper
If we then define $C$ as
#align(center)[
  $C = 1/2(abs(p_"b" - p_"a")^2 - l^2)$
]
where $p_a$ and $p_b$ are the positions of the objects. The $1/2$ and squares just make the derivation easier. Continuing on, we can set $p_"ab" = p_b - p_a$ which lets us simplify this into 
#align(center)[
  $C = 1/2(abs(p_"ab")^2 - l^2)$
]
This is fairly easy to handle and to derive:
#align(center)[
  $dot.basic(C) = dv(,t) 1/2 abs(p_"ab")^2\
  = p_"ab" dv(,t) p_"ab"\
  = (p_"b" - p_"a") dot.c (dv(,t)p_b - dv(,t)p_a)$
]
And we can get $dv(,t)p_i$ from the first derivative of position, or velocity of the objects:
#align(center)[
  $dot.basic(C) = (p_"b" - p_"a") dot.c (v_b - v_a)$
]
Then we need to factor out $v$ to get $J$:
#align(center)[
  $dot.basic(C) = (p_"b" - p_"a") dot.c v_b - (p_"b" - p_"a") dot.c v_a$
]
Meaning that if we take $v$ to be $vec(v_a,v_b)$ we get a Jacobian of $vec(-(p_b - p_a),(p_b - p_a)) ^ TT$. This is our constraint Jacobian for both objects in both dimensions. We can just ignore whichever side is connected to the mouse and only move one end of the constraint and we have a mouse following constraint.

#pagebreak()
#heading("Conclusion", bookmarked: true, depth: 1, outlined: true)
The way that this was implemented is simply the most Calculus IV reliant method I could find, since every constraint needs to only be defined by its Jacobian, but it isn't flawless. For one, Baumgarte Stabilization is quite difficult to get correct using this position constraint based solver. Baumgarte Stabilization allows the simulation to not just avoid breaking a constraint, but also restore a constraint if it gets broken, whether it be because of something that can't be controlled by physics (like user input) knocking things around in the simulation, or just round-off errors because of how computers do math. There are better solutions out there, which rely less heavily on Calculus IV and if you do find this interesting, I recommend looking through my sources and doing research on your own. I will most likely be exploring the topic more as well. Have fun!

#pagebreak()
#heading("Bibliography", bookmarked: true, depth: 1, outlined: true)
#bibliography("My Library 2.bib", full: true, style: "modern-language-association-8", title: none)
