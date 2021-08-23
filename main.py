from datetime import datetime
import random
import json

class record():
    
    def __init__(self) -> None:
        pass

    def addRecord(self,item,type):
        self.item = item
        self.type = type

        date = datetime.today()
        date = date.strftime("%d/%m/%Y")
        
        if self.type == "lunch":
            with open("lunchRecord.json","r") as fobj:
                content = json.load(fobj)
                fobj.close()

        elif self.type == "dinner":
            with open("dinnerRecord.json","r") as fobj:
                content = json.load(fobj)
                fobj.close()

        else:
            raise SyntaxError ("invalid 'type' argument")

        if date not in content:
            content[date] = self.item

        if self.type == "lunch":
            with open("lunchRecord.json","w") as fobj:
                json.dump(content,fobj,indent=6)
                fobj.close()
        
        elif self.type == "dinner":
            with open("dinnerRecord.json","w") as fobj:
                json.dump(content,fobj,indent=6)
                fobj.close()


def main():
    pass

main()

R = record()
R.addRecord("food0","dinner")
R.addRecord("food1","lunch")