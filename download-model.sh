#!/bin/bash

mkdir -p ~/models/all-MiniLM-L6-v2                                                                                                                                                                       
cd ~/models/all-MiniLM-L6-v2                                                                                                                                                                             
BASE=https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main                                                                                                                                    
for f in model.onnx tokenizer.json config.json special_tokens_map.json tokenizer_config.json; do                                                                                                         
  curl -fL -o "$f" "$BASE/$f"                                                   
done 
