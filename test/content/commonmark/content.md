Cupcake ipsum dolor sit amet. Soufflé apple pie pie halvah pastry jelly beans ice cream. Wafer tart topping pudding I love I love bonbon I love. Jelly beans macaroon sugar plum I love caramels tootsie roll. I love I love candy. Pie lemon drops dragée pastry. I love tootsie roll apple pie muffin muffin pudding I love donut.

- I love brownie gummi bears chocolate bar. 
- Carrot cake toffee ice cream wafer sweet sweet sesame snaps sweet. 
- Apple pie donut I love sweet apple pie dragée soufflé toffee danish. 

Icing chocolate cake cake [jujubes](https://twitter.com/dennybritz) I love carrot cake tootsie roll. Pie chocolate cake tiramisu chupa chups cheesecake I love **pudding**. Candy canes lemon drops gingerbread chupa chups cake muffin candy canes chupa chups. Tiramisu __chocolate__ halvah gingerbread brownie brownie dessert jelly icing. Bear claw I love danish sesame snaps donut candy canes donut. Gummi bears carrot cake *macaroon* soufflé jelly-o.

## Croissant fruitcake halvah icing candy canes tart pastry

Chupa chups macaroon gummi bears dessert dessert. Cheesecake danish I love I love cookie chupa chups. Bear claw marzipan cake bonbon cake I love cookie jelly-o. Sesame snaps sweet marshmallow I love jelly sesame snaps. Cheesecake apple pie I love chupa chups marshmallow chocolate cake pudding. Pudding marshmallow lemon drops croissant. Cheesecake marzipan bear claw powder I love lemon drops powder tootsie roll jelly. Danish I love muffin jelly dragée toffee jujubes.

> Caramels chocolate bar apple pie donut wafer I love. 
> Wafer muffin icing marshmallow candy marzipan jelly-o.
> Cotton candy wafer halvah cupcake. Gingerbread halvah caramels tart.

Gummies jelly gummies I love I love biscuit brownie. Dragée pie dessert I love cotton candy cookie chupa chups. 

### Liquorice liquorice tootsie roll ice cream. Chocolate macaroon powder.

Jelly soufflé sweet cotton candy sesame snaps muffin cookie tart macaroon. Tiramisu candy icing danish donut icing pastry. Tart icing cupcake I love bonbon soufflé I love I love fruitcake. Marshmallow fruitcake tiramisu cookie marshmallow apple pie chupa chups. Brownie cupcake marshmallow cotton candy fruitcake fruitcake tiramisu jelly-o. Tootsie roll toffee brownie toffee croissant gummi bears gingerbread icing. Candy lemon drops candy apple pie I love donut cookie. Brownie chupa chups lollipop.

```python
def train():
    model.train() # Turn on the train mode
    total_loss = 0.
    start_time = time.time()
    ntokens = len(TEXT.vocab.stoi)
    for batch, i in enumerate(range(0, train_data.size(0) - 1, bptt)):
        data, targets = get_batch(train_data, i)
        optimizer.zero_grad()
        output = model(data)
        loss = criterion(output.view(-1, ntokens), targets)
        loss.backward()
        torch.nn.utils.clip_grad_norm_(model.parameters(), 0.5)
        optimizer.step()

        total_loss += loss.item()
        log_interval = 200
        if batch % log_interval == 0 and batch > 0:
            cur_loss = total_loss / log_interval
            elapsed = time.time() - start_time
            print('| epoch {:3d} | {:5d}/{:5d} batches | '
                  'lr {:02.2f} | ms/batch {:5.2f} | '
                  'loss {:5.2f} | ppl {:8.2f}'.format(
                    epoch, batch, len(train_data) // bptt, scheduler.get_lr()[0],
                    elapsed * 1000 / log_interval,
                    cur_loss, math.exp(cur_loss)))
            total_loss = 0
            start_time = time.time()
```

Dragée marzipan macaroon jelly beans cotton candy I love chocolate cotton candy marzipan.

#### Muffin gingerbread cake jelly beans chocolate cake

Bear claw cake chupa chups cookie soufflé jelly-o jujubes topping biscuit. I love powder toffee caramels I love tart. Caramels lemon drops I love cake danish cupcake tiramisu. Fruitcake carrot cake jujubes. Danish I love bonbon. Chupa chups tart donut. Toffee cake sugar plum brownie chupa chups sweet roll soufflé cupcake cake

$$
e^x = \sum_{n=0}^\infty \frac{x^n}{n!} = \lim_{n\rightarrow\infty} (1+x/n)^n
$$

Candy cookie cupcake oat cake liquorice lollipop. Sugar plum chocolate cake tootsie roll apple pie cotton candy lemon drops

## Ice cream liquorice chocolate bar marzipan 

Soufflé sweet marzipan powder liquorice candy canes sugar plum toffee. Sesame snaps icing dessert cheesecake sweet roll. Donut sweet roll biscuit carrot cake ice cream muffin. Pastry bear claw jelly beans chocolate gummi bears toffee. Candy muffin ice cream cotton candy wafer. Carrot cake dragée liquorice lemon drops icing liquorice sweet roll. Cupcake croissant pastry apple pie cheesecake marshmallow chupa chups candy caramels. Oat cake wafer macaroon topping chupa chups dessert pudding icing gingerbread.

See [@platanios2020jelly] for more details.

Brownie donut cake marzipan pudding muffin dessert pastry ice cream. Wafer gummi bears topping jujubes gingerbread marshmallow ice cream. Cake jelly beans danish lemon drops jelly beans. Jelly beans cookie candy canes. Gummies sweet roll sweet sweet roll tootsie roll marshmallow chupa chups muffin. Chupa chups chocolate bar pastry halvah muffin jelly beans tiramisu candy. Bonbon jelly-o cheesecake dragée pastry pudding brownie. Brownie dessert tart.

![This is an image](assets/hello-md.png)

Croissant gingerbread chocolate caramels croissant cotton candy. Powder dessert dessert cheesecake pastry toffee cheesecake.