# Kubernetes deployment configs

This is NOT a production ready configuration. It's just here to serve as a demo which you can find actively running
here: [https://s57dev.mxmariner.com/styles/day_bright_style/#14/47.27888/-122.41757](https://s57dev.mxmariner.com/styles/day_bright_style/#14/47.27888/-122.41757).

Notes:

Ideally you would *NOT* bake the data into the container image like this. But this is just a demo so:
1st you'll want to adjust the urls in data/config.json and data/styles/day_bright_style.json
```shell script
docker build -t manimaul/us5wa22m:latest -f US5WA22M.Dockerfile .
docker push manimaul/us5wa22m:latest
cat k8s.yml | linkerd inject - | kubectl apply -f -
``` 
 