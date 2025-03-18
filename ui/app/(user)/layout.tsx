export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div className="w-full mx-auto sm:max-w-[640px] md:max-w-[768px] lg:max-w-[1024px] px-4 py-2">
      <div className="flex justify-end mb-4">
        <p className="text-sm text-muted-foreground">
          Are you an influencer?{" "}
          <a href="#" className="text-primary hover:underline">
            Create your stock!
          </a>
        </p>
      </div>

      {children}
    </div>
  );
}
