import subprocess

result = subprocess.run(["docker", "logs", "-n", "1", "octez-node"], stdout=subprocess.PIPE)
print(result.stdout)

