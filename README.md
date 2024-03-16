## Sparkling-Water-Challenge
This repository contains my attempt of the Lambda Sparkling Water Bootcamp problem  
I have tried to convert the given private key (hex string) to ```u64```  
Then this u64 is used to get the public key by using generator and ```fn operate_with_self()```  
This result is converted back to affine form by using ```to_affine()``` & the ```(x, y)``` coordinates are printed  
I have used the lambdaworks docs and some online articles since I was not familiar with the ```BLS-12-381``` curve  
