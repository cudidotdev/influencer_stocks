"use client";

import { useEffect, useState } from "react";
import { Clock, AlertCircle } from "lucide-react";
import { Card } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { cn } from "@/lib/utils";

interface CountdownTimerProps {
  endTime: string | Date;
}

export function CountdownTimer({ endTime }: CountdownTimerProps) {
  const [timeLeft, setTimeLeft] = useState<{
    hours: number;
    minutes: number;
    seconds: number;
    totalSeconds: number;
    percentageLeft: number;
  }>({
    hours: 0,
    minutes: 0,
    seconds: 0,
    totalSeconds: 0,
    percentageLeft: 100,
  });

  const [isExpired, setIsExpired] = useState(false);
  const [isUrgent, setIsUrgent] = useState(false);

  useEffect(() => {
    const endTimeDate = new Date(endTime);
    const auctionDuration = 24 * 60 * 60; // 24 hours in seconds

    const calculateTimeLeft = () => {
      const now = new Date();
      const difference = endTimeDate.getTime() - now.getTime();

      if (difference <= 0) {
        setIsExpired(true);
        return {
          hours: 0,
          minutes: 0,
          seconds: 0,
          totalSeconds: 0,
          percentageLeft: 0,
        };
      }

      const totalSeconds = Math.floor(difference / 1000);
      const hours = Math.floor(totalSeconds / 3600);
      const minutes = Math.floor((totalSeconds % 3600) / 60);
      const seconds = totalSeconds % 60;

      // Calculate percentage of time left (assuming 24-hour auction)
      const percentageLeft = Math.min(
        100,
        Math.max(0, (totalSeconds / auctionDuration) * 100),
      );

      // Set urgent flag if less than 1 hour remains
      setIsUrgent(hours < 1);

      return { hours, minutes, seconds, totalSeconds, percentageLeft };
    };

    // Initial calculation
    setTimeLeft(calculateTimeLeft());

    // Update every second
    const timer = setInterval(() => {
      setTimeLeft(calculateTimeLeft());
    }, 1000);

    return () => clearInterval(timer);
  }, [endTime]);

  return (
    <Card
      className={cn(
        "p-3 border-2 transition-colors duration-300 w-fit",
        isExpired
          ? "border-destructive bg-destructive/5"
          : isUrgent
            ? "border-orange-400 bg-orange-50 dark:bg-orange-950/20"
            : "border-primary/20 bg-primary/5",
      )}
    >
      <div className="flex items-center gap-3">
        {isExpired ? (
          <AlertCircle className="h-5 w-5 text-destructive" />
        ) : (
          <Clock
            className={cn(
              "h-5 w-5",
              isUrgent ? "text-orange-500" : "text-primary",
            )}
          />
        )}
        <div className="space-y-1">
          <div className="font-mono text-lg font-bold tracking-wider">
            {isExpired ? (
              <span className="text-destructive">ENDED</span>
            ) : (
              <span
                className={
                  isUrgent ? "text-orange-600 dark:text-orange-400" : ""
                }
              >
                {String(timeLeft.hours).padStart(2, "0")}:
                {String(timeLeft.minutes).padStart(2, "0")}:
                {String(timeLeft.seconds).padStart(2, "0")}
              </span>
            )}
          </div>
          <div className="w-full">
            <Progress
              value={timeLeft.percentageLeft}
              className={cn(
                "h-1.5",
                isExpired
                  ? "bg-destructive/20"
                  : isUrgent
                    ? "bg-orange-200 dark:bg-orange-900"
                    : "bg-primary/20",
              )}
            />
          </div>
          <p className="text-xs text-muted-foreground">
            {isExpired ? "Auction has ended" : "Auction ends in"}
          </p>
        </div>
      </div>
    </Card>
  );
}
