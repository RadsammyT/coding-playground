import curses

def main(stdscr):
    stdscr.addstr(0, 0, "Hello World!")
    stdscr.refresh()
    stdscr.getch()
    curses.endwin()

curses.wrapper(main)