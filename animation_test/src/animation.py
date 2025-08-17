import manim
import math as m

class Animation_Test(Scene):
    def contruct(self):
        axes = Axes()
        
        lambda_equation = lambda x: m.sin()
        plotted_function = axes.plot(lambda_equation)

        dot = Dot()


        self.play(FadeIn(Axes))
        self.play(Create(plotted_function))
        self.play(MoveAlongPath(dot, plotted_function))


