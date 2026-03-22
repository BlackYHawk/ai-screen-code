import { services } from '@/data/services'
import { ServiceCard } from './ServiceCard'

export function ServiceGrid() {
  return (
    <section id="services" className="py-16 bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
          {services.map((service) => (
            <ServiceCard
              key={service.id}
              service={service}
              isDeveloperFeature={service.type === 'developer'}
            />
          ))}
        </div>
      </div>
    </section>
  )
}
