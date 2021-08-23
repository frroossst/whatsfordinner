from datetime import datetime
import random
import json
import os

class Record():
    
    def __init__(self) -> None:
        lunchF = "lunchRecord.json"
        dinnerF = "dinnerRecord.json"
        if os.path.exists(lunchF):
            pass
        else:
            with open("lunchRecord.json","w") as fobj:
                fobj.close()
        if os.path.exists(dinnerF):
            pass
        else:
            with open("dinnerRecord.json","w") as fobj:
                fobj.close()

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

class algorithm():

    def __init__(self) -> None:
        pass

def main():
    
    print("1. Add a record")
    print("2. Recommend food")
    print("/quit")
    ch = int(input("enter choice : "))
    if ch == 1:
        R = Record()
        item = input("enter food : ")
        dayTime = input("time of the day : ")
        R.addRecord(item,dayTime)

    elif ch == 2:
        pass
    
    elif ch == "/quit":
        quit()
    
    else:
        print("Invalid choice")
        main()

main()