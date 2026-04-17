import { cn } from '@/lib/utils';

interface ThinkingPulseProps {
  compact?: boolean;
  className?: string;
}

export function ThinkingPulse({ compact = false, className }: ThinkingPulseProps) {
  const size = compact ? 'h-4 w-4' : 'h-6 w-6';
  const coreSize = compact ? 'h-1.5 w-1.5' : 'h-2 w-2';
  const orbitSize = compact ? 'h-1 w-1' : 'h-1.5 w-1.5';

  return (
    <span
      aria-hidden="true"
      className={cn('relative inline-flex flex-none items-center justify-center', size, className)}
    >
      <span className="absolute inset-0 rounded-full border border-primary/40 bg-primary/10 animate-pulse" />
      <span className={cn('rounded-full bg-primary shadow-[0_0_10px_hsl(var(--primary)/0.45)]', coreSize)} />
      <span
        className={cn(
          'absolute rounded-full bg-primary/90',
          orbitSize,
          'animate-[spin_1.6s_linear_infinite] [transform-origin:center_-8px]'
        )}
      />
      <span
        className={cn(
          'absolute rounded-full bg-primary/70',
          orbitSize,
          'animate-[spin_1.8s_linear_infinite_reverse] [transform-origin:center_8px]'
        )}
      />
      <span
        className={cn(
          'absolute rounded-full bg-primary/80',
          orbitSize,
          'animate-[spin_2.2s_linear_infinite] [transform-origin:-8px_center]'
        )}
      />
    </span>
  );
}
