#from PIL import Image, ImageDraw

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __str__(self):
        return '({}, {})'.format(self.x, self.y)

    def __repr__(self):
        return '({}, {})'.format(self.x, self.y)


class LineSeg:
    def __init__(self, p1, p2):
        self.p1 = p1
        self.p2 = p2

    def __str__(self):
        return '({}, {})'.format(self.p1, self.p2)
    def __repr__(self):
        return '({}, {})'.format(self.p1, self.p2)

def ccw(A,B,C):
    return (C.y-A.y) * (B.x-A.x) > (B.y-A.y) * (C.x-A.x)

# Return true if line segments AB and CD intersect
def intersect(A,B,C,D):
    return ccw(A,C,D) != ccw(B,C,D) and ccw(A,B,C) != ccw(A,B,D)

def intersect_lines(ls_a, ls_b):
    return intersect(ls_a.p1, ls_a.p2, ls_b.p1, ls_b.p2)


#im = Image.new('RGBA', (800, 800), (255, 255, 255, 255)) 
#draw = ImageDraw.Draw(im)



total_lines = 100000

print('setting up')

from random import randrange
lines = []
for i in range(total_lines):
    a = Point(randrange(800), randrange(800)) 
    b = Point(randrange(800), randrange(800)) 
    line = LineSeg(a, b)
    lines.append(line)
    #draw.line((a.x, a.y, b.x, b.y), fill=0)

#im.show()

print('go!')

count = 0
while len(lines) != 0: 
    if len(lines) % 999 == 0:
        print(len(lines))
    line = lines.pop()
    for other in lines:
        if intersect_lines(line, other):
            count += 1

print(count, " in ", total_lines)