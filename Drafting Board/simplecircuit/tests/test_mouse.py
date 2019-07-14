import curses 

screen = curses.initscr() 
#curses.noecho() 
curses.curs_set(0) 
screen.keypad(1) 
curses.mousemask(1)

screen.addstr("This is a Sample Curses Script\n\n") 

while True:
    event = screen.getch() 
    if event == ord("q"): break 
    if event == curses.KEY_MOUSE:
        _, mx, my, _, bstate = curses.getmouse()
        screen.addstr(str(bin(bstate)) + " ")

curses.endwin()