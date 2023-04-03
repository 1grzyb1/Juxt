mod engine;

fn main() {
    let result = engine::execute("{#script}
    console.log(context);
    function getPort() {
        return 80;
    }
{/script}
apiVersion: v1
kind: Service
metadata:
  name: my-service
spec:
  ports:
    {#each port in [0, 1, 2]}
    - protocol: TCP
      port: ${port}
      targetPort: ${getPort()}
    {/each}");
    println!("{}", result);
}