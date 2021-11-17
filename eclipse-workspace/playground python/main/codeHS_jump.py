'''
Created on Nov 2, 2021

@author: RadsammyT
'''
import pygame
import random

SCREEN_WIDTH = 505
SCREEN_HEIGHT = 370

PLAYER_COLOR = (255, 0, 0)
SKY_COLOR = (94, 177, 247)

class Player(pygame.sprite.Sprite):
    def __init__(self):
        super().__init__()
        width = 40
        height = 60
        self.image = pygame.Surface([width, height])
        self.image.fill((255, 1, 1))
        
        self.rect = self.image.get_rect()
        
        self.vx = 0
        self.vy = 0
        
        self.level = None
        
    def draw(self):
        self.image.fill((255, 1, 1))
    
    def update(self):
        self.calc_grav()
        
        self.rect.x += self.vx
        collisions = pygame.sprite.spritecollide(self, self.level.platform_list, False)
        for collision in collisions:
            if self.vx > 0:
                self.rect.right = collision.rect.left
            elif self.vx < 0:
                self.rect.left = collision.rect.right
        
        self.rect.y += self.vy
        collisions = pygame.sprite.spritecollide(self, self.level.platform_list, False)
        for collision in collisions:
            if self.vy > 0 and self.rect.bottom >= collision.rect.top:
                self.rect.bottom = collision.rect.top
            if self.vy < 0 and self.rect.top <= collision.rect.bottom:
                self.rect.top = collision.rect.bottom
                
            self.vy = 0
            
    def calc_grav(self):
        if self.vy == 0:
            self.vy = 1
        else:
            self.vy += .35
            
        if self.rect.y >= SCREEN_HEIGHT - self.rect.height and self.vy >= 0:
            self.vy = 0
            self.rect.y = SCREEN_HEIGHT - self.rect.height
    
    def jump(self):
        self.rect.y += 2
        platform_hit_list = pygame.sprite.spritecollide(self, self.level.platform_list, False)
        self.rect.y -= 2
 
        # If it is ok to jump, set our speed upwards
        if len(platform_hit_list) > 0 or self.rect.bottom >= SCREEN_HEIGHT:
            self.vy = -10
            
    def left(self):
        self.vx = -6
    
    def right(self):
        self.vx = 6
    
    def stop(self):
        self.vx = 0

class Platform(pygame.sprite.Sprite):
    def __init__(self, width, height):
        super().__init__()
        self.image = pygame.Surface([width, height])
        self.image.fill((165, 131, 90))
        self.rect = self.image.get_rect()
        
    def draw(self):
        self.image.fill((165, 131, 90))
        self.rect = self.image.get_rect()
        
class Level():
    def __init__(self, player):
        self.platform_list = pygame.sprite.Group()
        self.enemy_list = pygame.sprite.Group()
        self.player = player
        
        self.world_shift = 0
    
    def update(self):
        self.platform_list.update()
        self.enemy_list.update()
        
    def draw(self, screen):
        screen.fill(SKY_COLOR)
        self.platform_list.draw(screen)
    
    def shift(self, dy):
        self.world_shift += dy
        for platform in self.platform_list:
            platform.rect.y += dy
            
class Level_01(Level):
    """ Definition for level 1. """
 
    def __init__(self, player):
        """ Create level 1. """
 
        # Call the parent constructor
        Level.__init__(self, player)
 
        self.level_limit = -1000
 
        # Array with width, height, x, and y of platform
        level = [
            [1000, 1, 0, 370],
            [40, 20, 20, 300],
            [40, 20, 100, 200],
            [40, 20, 400, 140],
            [40, 20, 300, 80],
            [40, 20, 0, 40],
            [40, 20, 0, -100],
            [40, 20, 350, -160],
            ]
            
 
        # Go through the array above and add platforms
        for platform in level:
            block = Platform(platform[0], platform[1])
            block.rect.x = platform[2]
            block.rect.y = platform[3]
            block.player = self.player
            self.platform_list.add(block)
 
 
# Create platforms for the level
class Level_02(Level):
    """ Definition for level 2. """
 
    def __init__(self, player):
        """ Create level 1. """
 
        # Call the parent constructor
        Level.__init__(self, player)
 
        self.level_limit = -1000
 
        # Array with type of platform, and x, y location of the platform.
        level = [
                # [210, 30, 450, 360],
                #  [210, 30, 850, 265],
                #  [210, 30, 1000, 328],
                #  [210, 30, 1120, 175],
                 ]
 
        # Go through the array above and add platforms
        for platform in level:
            block = Platform(platform[0], platform[1])
            block.rect.x = platform[2]
            block.rect.y = platform[3]
            block.player = self.player
            self.platform_list.add(block)
            
def main():
    pygame.init()

    size = [SCREEN_WIDTH, SCREEN_HEIGHT]
    screen = pygame.display.set_mode(size, pygame.RESIZABLE)
 
    pygame.display.set_caption("Side-scrolling Platformer")
 
    # Create the player
    player = Player()
 
    # Create all the levels
    level_list = []
    level_list.append(Level_01(player))
    level_list.append(Level_02(player))
 
    # Set the current level
    current_level_no = 0
    current_level = level_list[current_level_no]
 
    active_sprite_list = pygame.sprite.Group()
    player.level = current_level
 
    player.rect.x = 100
    player.rect.y = SCREEN_HEIGHT - player.rect.height
    active_sprite_list.add(player)
 
    # Loop until the user clicks the close button.
    done = False
 
    # Used to manage how fast the screen updates
    clock = pygame.time.Clock()
 
    # -------- Main Program Loop -----------
    while not done:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                done = True
 
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_LEFT:
                    player.left()
                if event.key == pygame.K_RIGHT:
                    player.right()
                if event.key == pygame.K_UP:
                    player.jump()
 
            if event.type == pygame.KEYUP:
                if event.key == pygame.K_LEFT and player.vx < 0:
                    player.stop()
                if event.key == pygame.K_RIGHT and player.vx > 0:
                    player.stop()
 
        # Update the player.
        active_sprite_list.update()
 
        # Update items in the level
        current_level.update()
        
        if player.rect.top <= 100:
            diff = player.rect.top - 100
            player.rect.top = 100
            current_level.shift(-diff)
 
        # If the player gets near the bottom side
        if player.rect.bottom > 370:
            diff = 0 - player.rect.bottom
            player.rect.bottom = 0
            current_level.shift(diff)

        if player.rect.left <= 0:
            player.rect.left = 0
        
        if player.rect.right >= 505:
            player.rect.right = 505
 
        # ALL CODE TO DRAW SHOULD GO BELOW THIS COMMENT
        current_level.draw(screen)
        active_sprite_list.draw(screen)
 
        # ALL CODE TO DRAW SHOULD GO ABOVE THIS COMMENT
 
        # Limit to 60 frames per second
        clock.tick(60)
 
        # Go ahead and update the screen with what we've drawn.
        pygame.display.flip()
 
    # Be IDLE friendly. If you forget this line, the program will 'hang'
    # on exit.
    pygame.quit()
 
if __name__ == "__main__":
    main()