import random
import math
import socket

class LoadGenerator:
    """
    Generates baseline traffic with normal variance and injects anomalies.
    Now acts as a real TCP client generating bytes.
    """
    def __init__(self, base_traffic: float = 10.0, variance: float = 2.0, anomaly_prob: float = 0.05, anomaly_multiplier: float = 5.0):
        if math.isnan(base_traffic) or math.isinf(base_traffic):
            raise ValueError(f"Invalid base_traffic: {base_traffic}")
        if math.isnan(variance) or math.isinf(variance) or variance < 0.0:
            raise ValueError(f"Invalid variance: {variance}")
        if math.isnan(anomaly_prob) or math.isinf(anomaly_prob) or not (0.0 <= anomaly_prob <= 1.0):
            raise ValueError(f"Invalid anomaly_prob: {anomaly_prob}")
        if math.isnan(anomaly_multiplier) or math.isinf(anomaly_multiplier):
            raise ValueError(f"Invalid anomaly_multiplier: {anomaly_multiplier}")

        self.base_traffic = base_traffic
        self.variance = variance
        self.anomaly_prob = anomaly_prob
        self.anomaly_multiplier = anomaly_multiplier

    def generate(self) -> float:
        traffic = max(0.0, random.gauss(self.base_traffic, math.sqrt(self.variance)))
        is_anomaly = random.random() < self.anomaly_prob
        if is_anomaly:
            traffic *= self.anomaly_multiplier
        return max(0.0, traffic)

    def generate_deterministic(self, step: int, anomaly_step: int = -1) -> float:
        traffic = self.base_traffic
        if step == anomaly_step:
            traffic *= self.anomaly_multiplier
        return max(0.0, traffic)

    def blast_network(self, host: str, port: int, traffic_amount: float):
        """
        Actually connects to a real Node TCP socket and sends bytes.
        """
        if traffic_amount <= 0:
            return

        try:
            s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            s.settimeout(0.5)
            s.connect((host, port))
            # send payload representing traffic volume
            payload = b"X" * int(traffic_amount)
            s.sendall(payload)
            s.close()
        except Exception:
            pass
