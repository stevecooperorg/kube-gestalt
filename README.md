# kube-gestalt

A kubernetes API explorer that supports custom views over several elements (gestalts)

Here's the problem. When you are debugging a kubernetes application, sometimes you need to jump around a lot of different elements -- say, from a cronjob, to a job, to the pod running that job, to the node the pod is running on... Doing this with basic tools like `kubectl` can be painful, and tools like `k9s` are designed as a general purpose tool, not for debugging a specific application.

`kube-gestalt` is designed to help you see specific things about your application.  You need to teach it how your stuff works -- say, if you're running bulk processing, then you might want to teach it about cronjobs you have, and statefulsets that run the bulk processing, and the nodes that those statefulsets run on.  Then you can see all of those things at once, and jump between them easily.

# Todo

- absolutely everything

in more detail;

- ~create a cluster for local testing using KinD~. -- https://kind.sigs.k8s.io/docs/user/quick-start/
- create a basic web server which can connect to a kubernetes API
- create an example kube-gestalt app which monitors pods and nodes
- in the example, create a pod-node gestalt which shows the pod and details from the node it's running from
- create a basic UI which can show the gestalts, probably using something like HTMX for simplicity