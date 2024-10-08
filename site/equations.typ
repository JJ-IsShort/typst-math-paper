#import "template.typ": *

#show: project.with(
  title: "An Overview of Constraint Based Physics Solving", 
  authors: ("Jai Steinmetz",)
)

#heading("Introduction", bookmarked: true, depth: 1, outlined: true)
In this paper, I will try and explain how constraint based physics solvers work as well as trying to derive a few constraints and showcase their implementations. I decided on this as my topic because I have always like physics simulations but whenever I've tried to write my own I've had serious numerical stability issues and even when the simulations stayed stable for a bit they looked wrong. I recently found out why. I haven't found many resources on how to program a proper physics simulation, and even less easy to understand resources. I am hoping that putting together this paper and these simulations will both act as a resource for others and will help me understand these concepts better for future simulations I may want to write.

#pagebreak()
#heading("Background", bookmarked: true, depth: 1, outlined: true)
I should first discuss the surrounding topics before getting to the part that involves multi-variable calculus. One of the most important things in physics simulations is the integration method. I plan to use Verlet Integration. The details of how it works aren't really needed for this paper. Its main advantages are how easy it is to implement, how incredibly numerically stable it is (especially when compared to Euler Integration which is what I was using), and how it is able to easily and as a property of the method itself conserve momentum without having to explicitly write it into other parts of the physics implementation. 
