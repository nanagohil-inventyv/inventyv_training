// @ts-check

/**
 * Implement the classes etc. that are needed to solve the
 * exercise in this file. Do not forget to export the entities
 * you defined so they are available for the tests.
 */

export function Size(width = 80 , height = 60){
   this.width = width;
   this.height = height;
}

Size.prototype.resize = function (newWidth , newHeight){
   this.width = newWidth;
   this.height = newHeight;
}

export function Position(x = 0,y = 0){
  this.x = x;
  this.y = y;
}

Position.prototype.move = function(newX,newY){
  this.x = newX;
  this.y = newY;
}

export class ProgramWindow{
   constructor(){
    this.screenSize = new Size(800,600);
    this.size = new Size();
    this.position = new Position();
  }
   resize(newSize) {
        // Minimum bounds
        let width = Math.max(1, newSize.width);
        let height = Math.max(1, newSize.height);

        // Maximum bounds based on current position
        const maxWidth = this.screenSize.width - this.position.x;
        const maxHeight = this.screenSize.height - this.position.y;

        // Clip to maximum allowed size
        width = Math.min(width, maxWidth);
        height = Math.min(height, maxHeight);

        // Apply the resized dimensions
        this.size.width = width;
        this.size.height = height;
    }
    move(newPosition){
       let x = Math.max(0,newPosition.x);
       let y = Math.max(0,newPosition.y);
      
       const maxWidth = this.screenSize.width - this.position.x;
       const maxHeight = this.screenSize.height - this.position.y;

       const maxX  = this.screenSize.width  - this.size.width;
       const maxY = this.screenSize.height - this.size.height;

      x = Math.min(x,maxX);
      y = Math.min(y,maxY);

      this.position.x = x;
      this.position.y = y;

      
      
    }
}

export function changeWindow(programWindow){
     programWindow.resize(new Size(400,300));
     programWindow.move(new Position(100 , 150));
     return programWindow;
}