export const A = ({ href, children }: { href: string; children: any }) => (
  <a href={href} class="underline decoration-1 border-dashed">
    {children}
  </a>
);
