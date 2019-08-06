import curses
import time

symbols = ["←","↑","→","↓","↞","↟","↠","↡","↔","↕","↔","↕"," "]

class TextView:
    def __init__(self, controller, model):
        self.controller = controller
        self.model = model

        self.model.registerObserver(self)
        self.stdscr = curses.initscr()

        self.stdscr.nodelay(True) # Makes getch non-blocking
        curses.mousemask(curses.KEY_MOUSE) # Allows mouse clicks to register
        curses.curs_set(0) 
        curses.noecho()
        self.stdscr.keypad(True) 

        self.rotate_mode = False

    def notified(self, board, state):
        self.update_display(board, state)

        event = self.stdscr.getch() 
        if event == ord('q'):
            self.controller.quit()

        if event == ord('r'):
            self.rotate_mode = not self.rotate_mode

        if event == curses.KEY_MOUSE:
            _, mx, my, _, _ = curses.getmouse()

            x = mx // 2
            y = my

            if self.rotate_mode:
                self.controller.rotate_block(x, y)
            else:
                self.controller.click(x, y)



    def update_display(self, board, state):
        for y in range(len(board)):
            for x in range(len(board[0])):
                if True in state[y][x]:
                    self.stdscr.addstr(y, 2*x, symbols[board[y][x]])
                else:
                    self.stdscr.addstr(y, 2*x, symbols[board[y][x]], curses.A_DIM)

        self.stdscr.refresh()

    def dispose(self):
        curses.endwin()