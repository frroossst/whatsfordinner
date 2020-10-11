import random
import getpass
def passw():
    r=getpass.getpass("password : ")
    rp='4883'
    if r==rp:
        pass
    else:
        quit()
a=0
li=['mag','tuver','masoor','thebra','thepla/masala bhakri','paratha','vinu bhai khichdi','tuver ma dhokdi','chora','pav bhaji','chole','rajma','vaal','turiya patra','bhagat muthiya','vaal ni daard','daad bhaat','masala khichdi','khada masala khichdi','frankie','burito','chatpata aloo naan','ragda petis','idli sambhar','dosa','ringad papdi','dana ringad','dhokla','daad ma dhokdi']
passw()
print("1. one food item")
print("2. multiple food items")
print()
ch=int(input("enter choice : "))
if ch==1:
    print(random.choice(li))
elif ch==2:
    a=int(input("how many food choices? "))
    x=random.sample(li,a)
    print()
    for i in x:
        print(i)
