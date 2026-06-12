import random
import math

class LoadGenerator:
    """
    Generates baseline traffic with normal variance and injects anomalies.
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
        """
        Generates traffic for the current tick, occasionally injecting anomalies
        based on the anomaly probability.
        """
        traffic = max(0.0, random.gauss(self.base_traffic, math.sqrt(self.variance)))

        is_anomaly = random.random() < self.anomaly_prob
        if is_anomaly:
            traffic *= self.anomaly_multiplier
            
        return max(0.0, traffic)

    def generate_deterministic(self, step: int, anomaly_step: int = -1) -> float:
        """
        Deterministic generation, allowing injection of an anomaly at a specific step.
        """
        # For determinism without using random state, we just return base_traffic
        traffic = self.base_traffic
        if step == anomaly_step:
            traffic *= self.anomaly_multiplier
            
        return max(0.0, traffic)
