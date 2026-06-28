import math
import json

class Node:
    """
    Represents a Node acting according to Cellular Biology and Free Energy Principle.
    It expects a certain amount of traffic, accumulates incoming traffic, and calculates surprise
    when traffic deviates from the expectation.
    """
    def __init__(self, node_id: str, expected_traffic: float = 0.0, surprise_threshold: float = 10.0, surprise_ratio: float = 0.1):
        if math.isnan(expected_traffic) or math.isinf(expected_traffic):
            raise ValueError(f"Invalid expected_traffic: {expected_traffic}")
        if math.isnan(surprise_threshold) or math.isinf(surprise_threshold) or surprise_threshold < 0:
            raise ValueError(f"Invalid surprise_threshold: {surprise_threshold}")
        if math.isnan(surprise_ratio) or math.isinf(surprise_ratio) or surprise_ratio < 0:
            raise ValueError(f"Invalid surprise_ratio: {surprise_ratio}")

        self.node_id = node_id
        self.expected_traffic = expected_traffic
        self.surprise_threshold = surprise_threshold
        self.surprise_ratio = surprise_ratio
        self.current_traffic = 0.0
        self.surprise = 0.0

        print(json.dumps({"message": f"Node initialized: {node_id}"}))
        print(json.dumps({"message": f"Markov blanket defined for {node_id}"}))
        print(json.dumps({"message": f"Generative model started for {node_id}"}))

    def receive_traffic(self, source_id: str, amount: float):
        """
        Accumulates incoming traffic.
        """
        if amount < 0 or math.isnan(amount) or math.isinf(amount):
            raise ValueError(f"Invalid traffic amount: {amount}")
        new_traffic = self.current_traffic + amount
        if math.isinf(new_traffic):
            raise OverflowError("Traffic accumulation resulted in infinity")
        self.current_traffic = new_traffic

    def step(self):
        """
        Evaluates surprise based on accumulated traffic, triggers an autonomous action
        if surprise is high, and updates the expected traffic model.
        Resets current traffic for the next step.
        """
        self.surprise = abs(self.current_traffic - self.expected_traffic)

        # Determine autonomous action
        action = None
        threshold = max(self.surprise_threshold, self.expected_traffic * self.surprise_ratio)

        if self.surprise > threshold:
            print(json.dumps({"message": f"{self.node_id} high surprise / free energy spike detected"}))
            if self.current_traffic > self.expected_traffic:
                action = "spawn"
                print(json.dumps({"message": f"{self.node_id} action: spawning sub-node"}))
            else:
                action = "throttle"
                print(json.dumps({"message": f"{self.node_id} action: throttling connection"}))
        elif self.surprise == 0.0 and self.current_traffic > 0:
             print(json.dumps({"message": f"{self.node_id} surprise levels drop to baseline"}))

        print(json.dumps({"message": f"{self.node_id} Updating predictive model"}))

        # Update expected traffic using Exponential Moving Average
        alpha = 0.2
        self.expected_traffic = (alpha * self.current_traffic) + ((1 - alpha) * self.expected_traffic)

        # Reset traffic for next step
        self.current_traffic = 0.0
        return action

    def get_surprise(self) -> float:
        """
        Returns the calculated surprise (free energy) from the last step.
        """
        return self.surprise
